

var qElement = document.getElementById("question")
var date = randomDate()
qElement.innerText = question_str(date)


function question_str(d) {
    var options = { year: 'numeric', month: 'long', day: 'numeric' };
    return d.toLocaleString('en-GB', options);
}

function randomDate() {
    var startDay = new Date(2019, 1, 1);
    var endDay = new Date(2019, 11, 31);
    var randTime = (endDay.getTime() - startDay.getTime()) * Math.random() + startDay.getTime()
    var date = new Date(randTime);
    return date;
}

function onSelectAnwser(){
    var answer = document.getElementById("option")
    var response = document.getElementById("text")
    if (answer.value == date.getDay()) {
        response.innerText = "Correct";
    } else {
        response.innerText = "Wrong, anwser was " + date.toDateString();
    }
    console.log("Input comming")
    date = randomDate()
    qElement.innerText = question_str(date)
};