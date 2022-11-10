const table = document.getElementById("songs")
const headers = table.getElementsByTagName("TR")[0].children
const directions = [];

const sort = (n) => {
  let rows, i, x, y
  let switching = true
  const descending = direction(n)

  while (switching) {
    switching = false
    rows = table.rows

    for (i = 1; i < (rows.length - 1); i++) {
      a = rows[i].getElementsByTagName("TD")[n].innerHTML.toLowerCase()
      b = rows[i + 1].getElementsByTagName("TD")[n].innerHTML.toLowerCase()
      if ((descending && a < b) || (!descending && a > b)) {
        rows[i].parentNode.insertBefore(rows[i + 1], rows[i])
        switching = true
        break
      }
    }
  }
  directions[n] = !descending
}

const direction = (index) => {
  for (let header of headers) {
    delete header.dataset.sort
  }
  const value = directions[index] !== false
  headers[index].dataset.sort = value ? "desc" : "asc"
  return value
}
