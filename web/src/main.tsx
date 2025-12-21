import { RouterProvider } from "@tanstack/react-router";
import "./index.css";

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { router } from "./router";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./queryClient";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </StrictMode>
);
