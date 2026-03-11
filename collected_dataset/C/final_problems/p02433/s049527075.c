#include <stdio.h>
#include <limits.h>
#include <stdlib.h>

// Here we implement list in double linked list.

typedef struct _Node {
	int value;
	struct _Node *prev;
	struct _Node *next;
} Node;

typedef struct {
	Node *head;
	Node *cursor;
} List;

Node *createNode(int _value) {
	Node *node = (Node*)malloc(sizeof(Node));
	node->value = _value;
	node->prev = NULL;
	node->next = NULL;
	return node;
}

List *initList() {
	Node *end_node = createNode(INT_MAX);	// we set INT_MAX as the special element END
	List *list = (List*)malloc(sizeof(List));
	list->head = end_node;
	list->cursor = end_node;
	return list;
}

void disposeList(List *_list) {
	Node *tmp;
	if (_list != NULL) {
		for (tmp = _list->head; tmp != NULL; tmp = _list->head) {
			_list->head = tmp->next;
			free(tmp);
		}
	}
}

void insert(List *_list, int _x) {
	Node *new_node = createNode(_x);
	new_node->next = _list->cursor;
	if (_list->head == _list->cursor) {	// if cursor indicate the head
		_list->head = new_node;
	}
	else {
		new_node->prev = _list->cursor->prev;
		new_node->prev->next = new_node;
	}
	_list->cursor->prev = new_node;
	_list->cursor = new_node;
}

void move(List *_list, int _d) {
	if (_d > 0) {
		while (_d--) {
			_list->cursor = _list->cursor->next;
		}
	}
	else {
		while (_d++) {
			_list->cursor = _list->cursor->prev;
		}
	}
}

void erase(List *_list) {
	Node *tmp = _list->cursor;
	tmp->next->prev = tmp->prev;
	if (_list->head == _list->cursor) {
		_list->head = tmp->next;
	}
	else {
		tmp->prev->next = tmp->next;
	}
	_list->cursor = tmp->next;
	free(tmp);
}

void printList(List *_list) {
	Node *tmp = _list->head;
	while (tmp->value != INT_MAX) {
		printf("%d\n", tmp->value);
		tmp = tmp->next;
	}
}

int main() {
	int q;  // amount of queries
	int op; // number of operations
	int si; // the second input
	int i;  // index
	List *list = initList();
	scanf("%d", &q);
	for (i = 0; i != q; ++i) {
		scanf("%d", &op);
		switch (op) {
			case 0:
				scanf("%d", &si);
				insert(list, si);
				break;
			case 1:
				scanf("%d", &si);
				move(list, si);
				break;
			default:
				erase(list);
		}
	}
	printList(list);
	disposeList(list);
	return 0;
}
