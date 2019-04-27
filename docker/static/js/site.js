// TODO: TypeScript

const ENTER_KEY_CODE = 13;

document.addEventListener('DOMContentLoaded', init, false);

function init() {

  let year = parseInt(document.getElementById('year').value);
  let month = parseInt(document.getElementById('month').value);

  initNext(year, month);
  initPrevious(year, month);
  initWriteEvent(year, month);
}

function initNext(year, month) {

  let next_month = (month === 12) ? 1 : (month + 1);
  
  // Next year changes only if current month is December,
  //  otherwise 'next_year' is actually current year.
  let next_year = (month === 12) ? (year + 1) : year;
  let  addr_next = `/${next_year}/${next_month}`;
  let arrow_next = document.getElementById('arrow_next');

  arrow_next.addEventListener('click', function() {
    location.assign(addr_next);
  });
}

function initPrevious(year, month) {

  let previous_month = (month === 1) ? 12 : (month - 1);
  let previous_year = (month === 1) ? (year - 1) : year;
  let  addr_prev = `/${previous_year}/${previous_month}`;
  let arrow_previous = document.getElementById('arrow_previous');

  arrow_previous.addEventListener('click', function() {
    location.assign(addr_prev);
  });
}

function initWriteEvent(year, month) {

  let inputs = document.getElementsByTagName('input');
  
  Array.from(inputs).forEach(function(elem) {
    elem.addEventListener('keydown', function(e) {
      if (e.keyCode === ENTER_KEY_CODE) {

        let day = this.dataset.day;

        let event_value = this.value;

        let url = `/write-event/${year}/${month}/${day}`

        var xhr = new XMLHttpRequest();
        xhr.open('POST', url, true);

        xhr.send(event_value);

        xhr.onloadend = function () {
          let addr = `/${year}/${month}`;
          location.assign(addr);
        };

      }
    });
  });
}