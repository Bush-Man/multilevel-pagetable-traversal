
// Dummy data
const dummyData = {
  memWrites: [
    {
      timestamp: "2023-10-01 12:00",
      virtualAddress: "0x1000",
      physicalAddress: "0x2000",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic",
      status: "Success",
    },
    {
      timestamp: "2023-10-01 12:01",
      virtualAddress: "0x1001",
      physicalAddress: "0x2001",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic",
      status: "Success",
    },
    {
      timestamp: "2023-10-01 12:04",
      virtualAddress: "0x1004",
      physicalAddress: "0x2004",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic",
      status: "Error",
    },
    {
      timestamp: "2023-10-01 12:05",
      virtualAddress: "0x1005",
      physicalAddress: "0x2005",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic!",
      status: "Error",
    },
    {
      timestamp: "2023-10-01 12:04",
      virtualAddress: "0x1004",
      physicalAddress: "0x2004",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic!",
      status: "Error",
    },
    {
      timestamp: "2023-10-01 12:05",
      virtualAddress: "0x1005",
      physicalAddress: "0x2005",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic!",
      status: "Error",
    },
  ],
  memReads: [
    {
      timestamp: "2023-10-01 12:02",
      virtualAddress: "0x1002",
      physicalAddress: "0x2002",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic!",
      status: "Success",
    },
    {
      timestamp: "2023-10-01 12:03",
      virtualAddress: "0x1003",
      physicalAddress: "0x2003",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic!",
      status: "Success",
    },
  ],
  memErrors: [
    {
      timestamp: "2023-10-01 12:04",
      virtualAddress: "0x1004",
      physicalAddress: "0x2004",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic",
      status: "Error",
    },
    {
      timestamp: "2023-10-01 12:05",
      virtualAddress: "0x1005",
      physicalAddress: "0x2005",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic",
      status: "Error",
    },
    {
      timestamp: "2023-10-01 12:00",
      virtualAddress: "0x1000",
      physicalAddress: "0x2000",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic",
      status: "Success",
    },
    {
      timestamp: "2023-10-01 12:01",
      virtualAddress: "0x1001",
      physicalAddress: "0x2001",
      data: "Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic Lorem ipsum dolor sit amet consectetur adipisicing elit. Tempore, hic",
      status: "Success",
    },
  ],
};

// loads data on tab click
function loadData(tabId) {
  const logDisplay = document.getElementById("log-display");
  logDisplay.innerHTML = ""; 

  // Sets active tab
  document
    .querySelectorAll(".logger-tabs")
    .forEach((tab) => tab.classList.remove("active"));
  document.getElementById(tabId).classList.add("active");

  // Loads new content
  dummyData[tabId].forEach((log) => {
    const logItem = document.createElement("tr");
    let errorType = log.status === "Error" ? "error" : "success";
    logItem.className = "log-item";
    logItem.innerHTML = `
        <td class='tab-item timestamp'>${log.timestamp}</td>
        <td class='tab-item virtual-address'>${log.virtualAddress}</td>
        <td class='tab-item physical-address'>${log.physicalAddress}</td>
        <td class='tab-item data'>${log.data.substring(0, 55)}...</td>
        <td class='tab-item data'>${log.data.substring(0, 55)}...</td>
        <td class='tab-item ${errorType}'>${log.status}</td>
      `;
    logDisplay.appendChild(logItem);
  });
}

// Loads initial data
document.addEventListener("DOMContentLoaded", () => loadData("memWrites"));

