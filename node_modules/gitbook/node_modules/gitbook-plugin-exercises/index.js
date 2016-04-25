module.exports = {
    book: {
        assets: "./book",
        js: [
            "ace/ace.js",
            "ace/theme-tomorrow.js",
            "ace/mode-javascript.js",
            "exercises.js"
        ],
        css: [
            "exercises.css"
        ],
        html: {
            "body:end": function(options) {
                return '<script src="'+options.staticBase+'/plugins/gitbook-plugin-exercises/jsrepl/jsrepl.js" id="jsrepl-script"></script>';
            }
        }
    }
};