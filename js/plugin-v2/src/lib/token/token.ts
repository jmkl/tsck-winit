import { storage } from "uxp";
import { RootFolderType } from "../model";

// Constants
const STORAGE_KEYS = {
  ROOT_FOLDER: "token_root_folder",
  TEMP_FOLDER: "token_temp_folder",
  ACTIVE_MODEL_INDEX: "active_model_index",
} as const;

export class TokenHelper {
  private rootFolder: Entry | null = null;
  private comfyuiFolder: Entry | null = null;
  private smartObjectFolder: Entry | null = null;
  private texturesFolder: Entry | null = null;
  private tempFolder: Entry | null = null;
  private templateFolder: Entry | null = null;
  private initialized = false;

  constructor() {}

  getSmartObjectFolder(): Entry {
    return this.smartObjectFolder;
  }

  /**
   * Initialize root and temp folders
   */
  async init(): Promise<void> {
    if (this.initialized) {
      return;
    }

    try {
      [this.rootFolder, this.tempFolder] = await Promise.all([
        await this.getOrPickFolder(STORAGE_KEYS.ROOT_FOLDER),
        await this.getOrPickFolder(STORAGE_KEYS.TEMP_FOLDER),
      ]);

      this.initialized = true;
      this.texturesFolder = await this.rootFolder.getEntry(
        RootFolderType.TEXTURE,
      );
      this.smartObjectFolder = await this.rootFolder.getEntry(
        RootFolderType.SMARTOBJECT,
      );
    } catch (error) {
      throw error;
    }
  }

  /**
   * Ensure TokenHelper is initialized before use
   */
  private ensureInitialized(): void {
    if (!this.initialized) {
      throw new Error("TokenHelper not initialized. Call init() first.");
    }
  }

  async getComfyUIFolder(): Promise<Entry> {
    return new Promise(async (resolve, reject) => {
      if (!this.comfyuiFolder) {
        this.comfyuiFolder = await this.rootFolder.getEntry(
          RootFolderType.COMFYUI2026,
        );

        resolve(this.comfyuiFolder);
      } else {
        resolve(this.comfyuiFolder);
      }
    });
  }
  async getComfyUiInputFolder(): Promise<Entry> {
    return await (await this.getComfyUIFolder()).getEntry("input");
  }
  async getComfyUiOutputFolder(): Promise<Entry> {
    return await (await this.getComfyUIFolder()).getEntry("output");
  }

  async getTextureEntry(
    folder: string,
    filename: string,
  ): Promise<Entry | null> {
    this.ensureInitialized();
    if (!this.rootFolder) {
      console.error("Root folder not ");
      return null;
    }

    try {
      if (!this.texturesFolder) {
        this.texturesFolder = await this.rootFolder.getEntry(
          RootFolderType.TEXTURE,
        );
      }
      const textureFolder = await this.texturesFolder.getEntry(folder);
      const texture = await textureFolder.getEntry(filename);
      return texture || null;
    } catch (error) {
      return null;
    }
  }
  async getSmartObjectEntry(filename: string): Promise<Entry | null> {
    this.ensureInitialized();
    if (!this.rootFolder) {
      return null;
    }

    try {
      if (!this.smartObjectFolder) {
        this.smartObjectFolder = await this.rootFolder.getEntry(
          RootFolderType.SMARTOBJECT,
        );
      }
      const smartobject = await this.smartObjectFolder.getEntry(filename);
      return smartobject || null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Get template folder entry by name
   * @param templateName - Name of the template file/folder
   * @returns Entry for the template or null if not found
   */
  async getTemplateFor(templateName: string): Promise<Entry | null> {
    this.ensureInitialized();

    if (!this.rootFolder) {
      return null;
    }

    try {
      // Lazy load template folder
      if (!this.templateFolder) {
        this.templateFolder = await this.rootFolder.getEntry(
          RootFolderType.TEMPLATE,
        );
      }

      if (!this.templateFolder) {
        return null;
      }

      const templateEntry = await this.templateFolder.getEntry(templateName);
      return templateEntry || null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Get or pick a folder and store its token
   * @param key - Storage key for the folder token
   * @returns Folder entry
   */
  private async getOrPickFolder(key: string): Promise<Entry | null> {
    // Try to get existing folder from token
    const existing = await this.getFolderFromToken(key);
    if (existing) return existing;

    // Pick new folder and save token
    const folder = await this.pickFolder(key);
    if (!folder) return null;

    await this.saveFolderToken(key, folder);
    return folder;
  }

  /**
   * Prompt user to pick a folder
   * @returns Selected folder entry or null
   */
  private async pickFolder(key: string): Promise<Entry | null> {
    try {
      const folder = await storage.localFileSystem.getFolder({
        initialDomain: storage.domains.userDocuments,
      });
      return folder || null;
    } catch (error) {
      return null;
    }
  }

  /**
   * Get folder from stored persistent token
   * @param key - Storage key for the token
   * @returns Folder entry or null
   */
  private async getFolderFromToken(key: string): Promise<Entry | null> {
    try {
      const token = localStorage.getItem(key);
      if (!token) return null;

      const entry =
        await storage.localFileSystem.getEntryForPersistentToken(token);
      return entry?.isFolder ? entry : null;
    } catch (error) {
      // Clean up invalid token
      localStorage.removeItem(key);
      return null;
    }
  }

  /**
   * Save persistent token for a folder
   * @param key - Storage key
   * @param folder - Folder entry
   */
  private async saveFolderToken(key: string, folder: Entry): Promise<void> {
    try {
      const token = await storage.localFileSystem.createPersistentToken(folder);
      localStorage.setItem(key, token);
    } catch (error) {
      throw error;
    }
  }

  /**
   * Get root folder
   */
  getRootFolder(): Entry | null {
    this.ensureInitialized();
    return this.rootFolder;
  }

  /**
   * Get temp folder
   */
  getTempFolder(): Entry | null {
    this.ensureInitialized();
    return this.tempFolder;
  }

  /**
   * Get template folder (lazy loaded)
   */
  async getTemplateFolder(): Promise<Entry | null> {
    this.ensureInitialized();

    if (!this.templateFolder && this.rootFolder) {
      try {
        this.templateFolder = await this.rootFolder.getEntry(
          RootFolderType.TEMPLATE,
        );
      } catch (error) {
        return null;
      }
    }

    return this.templateFolder;
  }

  /**
   * Reset all cached folders and reinitialize
   */
  async reset(): Promise<void> {
    this.rootFolder = null;
    this.tempFolder = null;
    this.templateFolder = null;
    this.initialized = false;
    await this.init();
  }

  /**
   * Clear all stored tokens
   */
  clearStoredTokens(): void {
    Object.values(STORAGE_KEYS).forEach((key) => {
      localStorage.removeItem(key);
    });
  }

  /**
   * Check if TokenHelper is ready to use
   */
  isReady(): boolean {
    return (
      this.initialized && this.rootFolder !== null && this.tempFolder !== null
    );
  }
}
