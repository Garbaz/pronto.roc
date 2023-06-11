platform "pronto"
    requires {} { main: { init : Init, draw : Draw } }
    exposes [Pronto]
    packages {}
    imports [Pronto.{ Draw, Init }]
    provides [mainForHost]

mainForHost = main