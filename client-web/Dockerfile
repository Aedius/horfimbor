FROM nginx:stable-alpine

# copy the build artifact from the build stage
COPY /target/wasm32-unknown-unknown/release/deps/client_web.wasm /usr/share/nginx/html/wasm/

# copy the static file from static
COPY static/* /usr/share/nginx/html

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
