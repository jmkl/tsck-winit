export type TextureItem = {
  id?: number;
  name?: string;
  thumb?: string;
  category?: string;
};
export type TexturePaginationItem = {
  page: number;
  total_pages: number;
  items: Array<Texture>;
};
