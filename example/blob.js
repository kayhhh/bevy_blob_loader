/**
 * JS function we define in this example to create a blob URL from a fetched file.
 * This could be anything, such as taking a file as input from the user.
 */
export async function get_blob() {
  const res = await fetch("/assets/drip.png");
  const blob = await res.blob();
  const url = URL.createObjectURL(blob);
  return url;
}
