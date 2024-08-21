<template>
    <table id="tree">
        <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Date Uploaded</th>
            <th>Preview</th>
            <th>Download</th>
        </tr>
    </table>
    <html id="preview-contents">

    </html>
</template>
<script>
import axios from "axios";
import DOMPurify from "dompurify";
import { marked } from "marked";
let get_route = process.env.NODE_ENV == "production" ? `${window.location.origin}/api` : `http://127.0.0.1:3000/api`;


function retrieve_files() {
    axios.get(`${get_route}/file/all`)
        .then(response => {
            console.log(response)

            for (let i in response.data) {
                let tree = document.getElementById("tree")
                let row = document.createElement("tr");
                let file_id = document.createElement("td");
                let name = document.createElement("td");
                let file_date = document.createElement("td");
                let file_name = response.data[i].filename;
                let upload_date = response.data[i].upload_time.split(" ")[0]
                let download_button = document.createElement("button");
                download_button.innerHTML = "Download";
                download_button.addEventListener("click", () => {
                    axios.get(`${get_route}/file/download/${i}`)
                        .then(response2 => {
                            /* create obj url with the blob recieved in response */
                            const url = window.URL.createObjectURL(new Blob([response2.data]));
                            /*  create new anchor tag */
                            const link = document.createElement('a');
                            /* set the location of the link to the url and set it to download with filename markdown */
                            link.href = url;
                            link.setAttribute('download', file_name);
                            /* add the link tag to the body */
                            document.body.appendChild(link);
                            /* click the link to trigger download */
                            link.click();
                            /* remove the link from page */
                            document.body.removeChild(link);
                            window.URL.revokeObjectURL(url);
                        })
                })
                let preview_button = document.createElement("button")
                preview_button.innerHTML = "Preview";
                preview_button.addEventListener("click", () => {
                    axios.get(`${get_route}/file/download/${i}`)
                        .then(response => {
                            let file_contents = response.data
                            tree.style.visibility = "hidden";
                            document.getElementById("preview-contents").innerHTML = DOMPurify.sanitize(marked.parse(file_contents));
                            console.log(file_contents)
                            document.getElementById("preview-contents").classList.add("center-content-markdown");
                        })
                })


                file_id.innerHTML = i;
                row.appendChild(file_id);
                tree.appendChild(row);
                name.innerHTML = file_name;
                row.appendChild(name);
                tree.appendChild(row);
                file_date.innerHTML = upload_date;
                row.appendChild(file_date);
                tree.appendChild(row);
                row.appendChild(preview_button);
                tree.appendChild(row);
                row.appendChild(download_button);
                tree.appendChild(row);


            }
        })
        .catch(error => {
            console.error(error)
        })
}
retrieve_files();

</script>
<style scoped>
* {
    font-family: "Poppins", sans-serif;
}

.error-message-hide {
    transition: all 0.5s ease-in-out;
    opacity: 0;
}

.error-message-show {
    opacity: 1;
}

#file-download-button {
    transition: background-color 0.5s ease-in-out;
    border: 1px solid #ccc;
    background-color: #ffff;
    display: inline-block;
    padding: 6px 12px;
    cursor: pointer;
    margin-top: 10px;
}

#file-download-button:hover {
    border: 1px solid #000000;
}


.center-content {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
}

.center-content-markdown {
    background-color: #FFFFFF;
    display: flex;
    justify-content: center;
    /* text-align: center; */
    margin: 0 10% 10% 10%;

}
</style>
