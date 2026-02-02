export type ServerResponse = {
  ok: boolean;
  data: TodoType[];
};

export type TodoType = {
  sender: string;
  status: TodoStatusType;
  message: string;
  old_message: string;
  time: string;
  messageId: string;
};

export const TodoStatus = {
  Default: 0,
  Done: 1,
  Edited: 2,
  Deleted: 3,
} as const;
export type TodoStatusType = (typeof TodoStatus)[keyof typeof TodoStatus];

export class TodoHelper {
  SERVER_URL: string;
  TODO_WHITELIST = [
    "todo@apps",
    "628119011970-1620112335@g.us" /*RH GROUP*/,
    "62895401388700-1582799172@g.us" /*TEMP GROUP*/,
  ];

  constructor(url: string) {
    this.SERVER_URL = url;
  }
  async fetchTodo(): Promise<TodoType[]> {
    const result = await fetch(this.SERVER_URL + "/todolist");
    if (result.ok) {
      const response: ServerResponse = await result.json();
      console.log(response);
      return response.data
        .filter(
          (dt) =>
            this.TODO_WHITELIST.includes(dt.sender) &&
            dt.message &&
            dt.message.length > 30 &&
            dt.message &&
            this.isJudulThumbnail(dt.message),
        )
        .reverse();
    } else {
      return [];
    }
  }
  isJudulThumbnail(text: string, limit = 10): boolean {
    // Keep only uppercase A–Z letters
    const onlyUpper = text.replace(/[^A-Z]/g, "");
    return onlyUpper.length > limit;
  }
  async deleteAll(): Promise<boolean> {
    return new Promise(async (resolve, reject) => {
      const result = await fetch(this.SERVER_URL + "/delete-todo", {
        method: "POST",
      });
      if (result.ok) {
        const response: ServerResponse = await result.json();
        resolve(response.ok);
      } else {
        reject(69);
      }
    });
  }

  async addTodoFromClipboard(text: string) {
    // const cleaned = text
    //   .trim()
    //   .replace(/(\[.+?\]\s.+?[0-9A-Za-z]:\s)/gm, "\n")
    //   .split("\n")
    //   .filter(Boolean);
    return new Promise(async (resolve, reject) => {
      console.log(text);
      const response = await fetch(this.SERVER_URL + "/add", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },

        body: JSON.stringify({
          content: text,
        }),
      });
      if (response.ok) resolve(response);
      else {
        reject(69);
      }
    });
  }
  async updateStatus(status: TodoStatusType, messageId: string) {
    const response = await fetch(this.SERVER_URL + "/status", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        status: status,
        messageId: messageId,
      }),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    return data;
  }
  deleteTodo(todoId: string) {}
}
