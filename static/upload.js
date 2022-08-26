"use strict";

function _(elem) {
    return document.getElementById(elem);
}

// https://stackoverflow.com/questions/8006715/drag-drop-files-into-standard-html-file-input
let dragdrop = document.getElementById("dragdrop");
dragdrop.ondragover = dragdrop.ondragenter = function(evt) {
    evt.preventDefault();
};

dragdrop.ondrop = function(evt) {
    _("file").files = evt.dataTransfer.files;

    const dT = new DataTransfer();
    dT.items.add(evt.dataTransfer.files[0]);
    dT.items.add(evt.dataTransfer.files[3]);
    file.files = dT.files;

    evt.preventDefault();
};

// https://codepen.io/PerfectIsShit/pen/zogMXP
function upload_file() {
    var file = _("file").files[0];

    // TODO:
    // Get max file size from /max_size
    // If file.size > max, display error right away.

    var formdata = new FormData();
    formdata.append("file", file);

    var ajax = new XMLHttpRequest();
    ajax.upload.addEventListener("progress", progress_handler, false);
    ajax.addEventListener("load", complete_handler, false);
    ajax.addEventListener("error", error_handler, false);
    ajax.addEventListener("abort", abort_handler, false);
    ajax.open("POST", "/upload");
    ajax.send(formdata);
}

function progress_handler(event) {
    var percent = (event.loaded / event.total) * 100;
    _("bar").value = Math.round(percent);
    _("upload_status").classList.add("msg-INFO");
    _("upload_status").innerHTML = Math.round(percent) + "% uploaded... please wait";
}

function complete_handler(event) {

    if(this.status === 413) {
        error_handler(event);
        return;
    }

    _("upload_status").classList.remove("msg-INFO");
    _("upload_status").classList.add("msg-OK");
    _("upload_status").innerHTML = event.target.responseText;
}

function error_handler(event) {
    _("upload_status").classList.remove("msg-INFO");
    _("upload_status").classList.add("msg-ERROR");
    let text = event.target.responseText != "" ? event.target.responseText : "Upload failed";
    _("upload_status").innerHTML = text;
}

function abort_handler(_event) {
    _("upload_status").classList.remove("msg-INFO");
    _("upload_status").classList.add("msg-ERROR");
    _("upload_status").innerHTML = "Upload Aborted";
}