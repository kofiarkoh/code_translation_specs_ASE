#include <stdio.h>
#define N 400000    // the max length of deque

typedef struct {
    int queue[N];
    int head;
    int tail;
    int length;
} Deque;

void push(Deque *_dq, int _d, int _x) {
    if (_dq->length == 0) {
        _dq->queue[_dq->head] = _x;
    }
    else {
        if (_d == 0) {  // add new element at the beginning of deque
            _dq->head = (N + _dq->head - 1)%N;
            _dq->queue[_dq->head] = _x;
        }
        else {          // _d == 1, add new element at the end of deque
            _dq->tail = (N + _dq->tail + 1)%N;
            _dq->queue[_dq->tail] = _x;
        }
    }
    ++_dq->length;
}

void randomAccess(Deque *_dq, int _p) {
    printf("%d\n", _dq->queue[(_dq->head + _p)%N]);
}

void pop(Deque *_dq, int _d) {
    if (_dq->head != _dq->tail) {
        if (_d == 0) {	// delete the first element
            _dq->head = (N + _dq->head + 1)%N;
        }
        else {			// _d == 1, delete the last element
            _dq->tail = (N + _dq->tail - 1)%N;
        }
    }
    --_dq->length;
}

int main() {
    int q;	// amount of querries
	int op;	// number of operation, 0: push, 1: randomAccess, 2: pop
	int si;	// the second input
	int x;	// input value
	int i;	// index
	Deque dq;
    dq.head = 0;
    dq.tail = 0;
    dq.length = 0;
	scanf("%d", &q);
	for (i = 0; i != q; ++i) {
		scanf("%d%d", &op, &si);
		switch (op) {
			case 0:		// push()
				scanf("%d", &x);
				push(&dq, si, x);
				break;
			case 1:		// randomAccess()
				randomAccess(&dq, si);
				break;
			default:	// op == 2, pop()
				pop(&dq, si);
		}
	}
	return 0;
}
