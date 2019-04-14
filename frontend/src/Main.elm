port module Main exposing (main)

import Browser
import Html exposing (Html)
import Html.Attributes exposing (placeholder, value)
import Html.Events exposing (onInput, onSubmit)
import Json.Decode exposing (Decoder, decodeString, field, map, oneOf, string)


port fromSocket : (String -> msg) -> Sub msg


port toSocket : String -> Cmd msg


main =
    Browser.element
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }



{- MODEL -}


type alias Model =
    { responses : List String
    , input : String
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( { responses = []
      , input = ""
      }
    , Cmd.none
    )



{- UPDATE -}


type Msg
    = Change String
    | Submit String
    | WebsocketIn String


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Change input ->
            ( { model | input = input }
            , Cmd.none
            )

        Submit value ->
            ( model
            , toSocket value
            )

        WebsocketIn value ->
            ( { model | responses = value :: model.responses }
            , Cmd.none
            )



{- SUBSCRIPTIONS -}


subscriptions : Model -> Sub Msg
subscriptions model =
    fromSocket decodeMessage


type Response
    = ResponseSuccess String
    | ResponseError String


responseDecoder : Decoder Response
responseDecoder =
    oneOf [ responseSuccessDecoder, responseErrorDecoder ]


responseSuccessDecoder : Decoder Response
responseSuccessDecoder =
    map ResponseSuccess (field "message" string)


responseErrorDecoder : Decoder Response
responseErrorDecoder =
    map ResponseError (field "reason" string)


decodeMessage : String -> Msg
decodeMessage x =
    let
        result =
            decodeString responseDecoder (Debug.log "elm json value" x)
    in
    case result of
        Ok response ->
            case response of
                ResponseSuccess message ->
                    WebsocketIn message

                ResponseError message ->
                    WebsocketIn message

        Err _ ->
            WebsocketIn "error :("



{- VIEW -}


li : String -> Html Msg
li string =
    Html.li [] [ Html.text string ]


view : Model -> Html Msg
view model =
    Html.div []
        [ Html.form [ onSubmit (Submit model.input) ]
            [ Html.input [ placeholder "Enter some text.", value model.input, onInput Change ] []
            , model.responses |> List.map li |> Html.ol []
            ]
        ]
