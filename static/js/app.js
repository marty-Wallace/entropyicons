

var host = window.location.origin + '/api/';
var name = 'marty@icons.mvw.io';
var itemset = 'circle';

function setup() {

    name = $('#name').val();
    itemset = $('#itemset').find('option:selected').val();
    setRadius($('#radius').val());

    // name event handler
    $('#name').on('input', _.throttle(function() {
        name = $(this).val();
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
}

function buildURL() {
    return '' + host + itemset + '/' + name;
}

function setURL() {
    $('#image').attr('src', buildURL());
}

window.addEventListener('load', function() {
    console.log('Running setup');
    setup();
}, false);
