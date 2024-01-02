export function get_blob() {
  const image = new Image();
  const url = URL.createObjectURL(image);
  console.log(url);
  return url;
}
