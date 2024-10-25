import {
  EditBufferChangedNotification,
  NotificationType,
} from "@amzn/fig-io-proto/fig";
import { _subscribe, NotificationResponse } from "./notifications.js";

export function subscribe(
  handler: (
    notification: EditBufferChangedNotification,
  ) => NotificationResponse | undefined,
) {
  return _subscribe(
    { type: NotificationType.NOTIFY_ON_EDITBUFFFER_CHANGE },
    (notification) => {
      switch (notification?.type?.$case) {
        case "editBufferNotification":
          return handler(notification.type.editBufferNotification);
        default:
          break;
      }

      return { unsubscribe: false };
    },
  );
}
