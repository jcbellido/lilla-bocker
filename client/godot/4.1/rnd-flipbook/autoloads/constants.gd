extends Node

const SERVER: String = "http://knuckles.lillaost.com:8888/"
const APIENDPOINT: String = "api/"
const BINENDPOINT: String = "flipbooks/"

func _ready():
	print_debug("Constants loaded")

func construct_url_of(path: String) -> String:
	return SERVER + BINENDPOINT + path

func url_get_all_v1() -> String:
	return SERVER + APIENDPOINT + "all-v1"
