
var host = window.location.origin + '/api/';
var static_host = window.location.origin + '/static/';
var name = 'Boxi@icons.mvw.io';
var itemset = 'shapes';

function setup() {

    setName(name);
    name = encodeURIComponent(name);
    setRadius($('#radius').val());

    // name event handler
    $('#name').on('input', _.throttle(function() {
        name = encodeURIComponent($(this).val());
        setValues();
        setURL();
    }, 99));

    // radius event handler
    $('#radius').on('input', _.throttle(function() {
        setRadius($(this).val());
    }, 99));

    // itemset event handler
    $('#itemset').on('change', function() {
        itemset = $('option:selected', this).val();
        setValues();
        setURL();
    });
    setURL();
    setValues();

    // smooth scrolling
     $(document).on('click', 'a[href^="#"]', function (event) {
         event.preventDefault();

         $('html, body').animate({
             scrollTop: $($.attr(this, 'href')).offset().top
         }, 500);
     });

     setInterval(function() {
        $('#header-image').attr('src', randomSquareImage());
     }, 750);
}

function randomSquareImage() {
    /// these are precomputed as static images
    var text = "";
    var possible = "01";
    for(var i = 0; i < 3; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }

    text = (Math.floor(Math.random() *100) % 2 ? 's': 'c') + text + '.png';
    return static_host + 'img/headers/' + text;
}

function setRadius(rad) {
    $('#host-value').text(host);
    $('#radius-value').text(' ' + rad);
    $('#image').css({
        'border-radius': '' + rad + '%'
    });
}

function setValues() {
    $('#itemset-value').text(itemset);
    $('#name-value').text(name);
    if(name === '') {
        $('#url-description').addClass('invalid');
    } else {
        $('#url-description').removeClass('invalid');
    }
}

function buildURL() {
    return '' + host + itemset + '/' + name;
}

function setURL() {
    if(name === '') {
        $('#image').attr('src', static_host + 'img/default2.png');
    }else {
        $('#image').attr('src', buildURL());
    }
}

function setName(name) {
    $('#name').val(name);
}

window.addEventListener('load', function() {
    console.log('Running setup');
    setup();
}, false);
