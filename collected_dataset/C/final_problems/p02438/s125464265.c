#include <stdio.h>
#include <stdlib.h>

typedef struct _Node {
	int value;
	struct _Node *next;
} Node;

typedef struct {
	int length;
	Node *head;
	Node *tail;
} List;

Node *CreateNode(int _value) {
	Node *newNode = (Node*)malloc(sizeof(Node));
	newNode->value = _value;
	newNode->next = NULL;
	return newNode;
}

List *InitListSequence(int _n) {
	int i;
	List *listSequence = (List*)malloc(sizeof(List)*_n);
	for (i = 0;i != _n;++i) {
		listSequence[i].length = 0;
		listSequence[i].head = NULL;
		listSequence[i].tail = NULL;
	}
	return listSequence;
}

// insert _value at the end of list of which subscript is _subscript
void Insert(List *_listSequence,int _subscript,int _value) {
	Node *newNode = CreateNode(_value);
	if (_listSequence[_subscript].length == 0) { // if the list is empty
		_listSequence[_subscript].head = newNode;
	}
	else {
		_listSequence[_subscript].tail->next = newNode;
	}
	_listSequence[_subscript].tail = newNode;
	++_listSequence[_subscript].length;
}

// print all elements in the list of which subscript is _subscript
void Dump(List *_listSequence,int _subscript) {
	Node *current;
	if (_listSequence[_subscript].length != 0) { // if the list isn't empty
		current = _listSequence[_subscript].head;
		while (current != NULL) {
			printf("%d",current->value);
			current = current->next;
			if (current != NULL) {
				printf(" ");
			}
		}
	}
	printf("\n");
}

// splice the list of which subscript is _subscript1 to the end of the list of which subscript is _subscript2
void Splice(List *_listSequence,int _subscript1,int _subscript2) {
	if (_listSequence[_subscript2].length == 0) { // if the list of which subscript is _subscript2 is empty
		_listSequence[_subscript2].head = _listSequence[_subscript1].head;
	}
	else {
		_listSequence[_subscript2].tail->next = _listSequence[_subscript1].head;
	}
	_listSequence[_subscript2].tail = _listSequence[_subscript1].tail;
	_listSequence[_subscript1].head = NULL;
	_listSequence[_subscript1].tail = NULL;
	_listSequence[_subscript2].length += _listSequence[_subscript1].length;
	_listSequence[_subscript1].length = 0;
}

int main() {
	int n;		// amount of lists
	int q;		// amount of queries
	int op;		// number of operation
	int first;	// the first parameter
	int second;	// the second parameter
	List *listSequence = NULL;
	scanf("%d%d",&n,&q);
	listSequence = InitListSequence(n);
	while (q--) {
		scanf("%d",&op);
		switch (op) {
			case 0:
				scanf("%d%d",&first,&second);
				// first:  subscript of list
				// second: the integer which will be inserted
				Insert(listSequence,first,second);
				break;
			case 1:
				scanf("%d",&first);
				// first:  subscript of list
				Dump(listSequence,first);
				break;
			default:
				scanf("%d%d",&first,&second);
				// first:  subscript of list s
				// second: subscript of list t
				Splice(listSequence,first,second);
		}
	}
	return 0;
}
