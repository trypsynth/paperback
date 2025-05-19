#include "app.hpp"
#include "main_window.hpp"
#include "parser_registry.hpp"

bool app::OnInit() {
	register_parsers();
	main_window* frame = new main_window();
	frame->Show(true);
	return true;
}

wxIMPLEMENT_APP(app);
