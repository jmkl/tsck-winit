export type SocketEventMap = {
  open: Event;
  close: Event;
  "payload|server": Event;
  error: CustomEvent<any>;
  message: CustomEvent;
};
