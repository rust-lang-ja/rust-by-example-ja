module.exports = {
    book: {
        assets: "./book",
        js: [
            "test.js"
        ],
        css: [
            "test.css"
        ]
    },
    hooks: {
        init: function() {
            console.log("init!");
        },
        finish: function() {
            console.log("finish!");
        }
    }
};