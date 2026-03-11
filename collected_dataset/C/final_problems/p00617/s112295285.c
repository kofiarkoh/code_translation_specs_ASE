#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#if 0
#define PRINT_TAGS
#endif

typedef struct panel_t_tag {
	char* name;
	int x1, y1, x2, y2;
	struct panel_t_tag* child;
	struct panel_t_tag* next;
} panel_t;

int my_ungetchar_num = 0;
int my_ungetchar_buf[1024];
int my_getchar(void) {
	if (my_ungetchar_num > 0) return my_ungetchar_buf[--my_ungetchar_num];
	return getchar();
}
void my_ungetchar(int c) {
	my_ungetchar_buf[my_ungetchar_num++] = c;
}

panel_t* parse(void) {
	char tag_name[1024];
	int input;
	panel_t* new_panel;
	input = my_getchar();
	if (input != '<') return NULL;
	new_panel = malloc(sizeof(panel_t));
	if (new_panel == NULL) exit(1);
	input = my_getchar();
	ungetc(input, stdin);
	if (input == '/') {
		my_ungetchar('<');
		return NULL;
	}
	if (scanf("%[^>]", tag_name) != 1) exit(1);
	input = my_getchar(); assert(input == '>');
	new_panel->name = malloc(strlen(tag_name) + 1);
	if (new_panel->name == NULL) exit(1);
	strcpy(new_panel->name, tag_name);
	if (scanf("%d,%d,%d,%d", &new_panel->x1, &new_panel->y1, &new_panel->x2, &new_panel->y2) != 4) exit(1);
	input = my_getchar(); assert(input == '<');
	input = my_getchar();
	if (input == '/') {
		if (scanf("%[^>]", tag_name) != 1) exit(1);
		assert(strcmp(new_panel->name, tag_name) == 0);
		input = my_getchar(); assert(input == '>');
		new_panel->child = NULL;
	} else {
		ungetc(input, stdin);
		my_ungetchar('<');
		new_panel->child = parse();
		input = my_getchar();
		if (input == '<') {
			if (scanf("%[^>]", tag_name) != 1) exit(1);
			assert(tag_name[0] == '/' && strcmp(new_panel->name, tag_name + 1) == 0);
			input = my_getchar(); assert(input == '>');
		}
	}
	new_panel->next = parse();
	return new_panel;
}

void free_panel(panel_t* panel) {
	if (panel == NULL) return;
	free(panel->name);
	free_panel(panel->child);
	free_panel(panel->next);
	free(panel);
}

#ifdef PRINT_TAGS
void print_tags(const panel_t* panel, int level) {
	int i;
	if (panel == NULL) return;
	for (i = 0; i < level; i++) printf("  ");
	printf("%s %d %d %d %d\n", panel->name, panel->x1, panel->y1, panel->x2, panel->y2);
	print_tags(panel->child, level + 1);
	print_tags(panel->next, level);
}
#else
int touch(const panel_t* panel, int x, int y) {
	int ret = 0;
	for (; panel != NULL; panel = panel->next) {
		if (panel->x1 <= x && x <= panel->x2 && panel->y1 <= y && y <= panel->y2) {
			int nret = touch(panel->child, x, y);
			if (nret >= 0) printf("%s %d\n", panel->name, nret);
			return -1;
		}
		ret++;
	}
	return ret;
}
#endif

int main(void) {
	char n_str[1024];
	int n, i;
	panel_t* root;
	while (fgets(n_str, sizeof(n_str), stdin) != NULL && sscanf(n_str, "%d", &n) == 1 && n > 0) {
		root = parse();
#ifdef PRINT_TAGS
		print_tags(root, 0);
#else
		for (i = 0; i < n; i++) {
			int x, y;
			if (fgets(n_str, sizeof(n_str), stdin) == NULL || sscanf(n_str, "%d%d", &x, &y) != 2) return 1;
			if (touch(root, x, y) >= 0) {
				puts("OUT OF MAIN PANEL 1");
			}
		}
#endif
		free_panel(root);
	}
	return 0;
}