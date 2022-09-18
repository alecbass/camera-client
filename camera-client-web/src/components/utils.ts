export function downloadFile(file: File, name: string) {
  const a = document.createElement("a");
  a.href = URL.createObjectURL(file);
  a.download = name;
  a.target = "_blank";
  document.body.append(a);
  a.click();
  a.remove();
}
