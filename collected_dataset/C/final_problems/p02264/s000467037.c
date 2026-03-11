#define _CRT_SECURE_NO_WARNINGS
#define MAX 50010;	//マクロは条件式内では使えない？
#include<stdio.h>
#include<stdlib.h>
#include<string.h>

struct process {
	char name[10];
	int time;
};


int max = MAX
struct process Q[50000];
int head, tail;

void initialize() {
	head = tail = 0;
}

int isEmpty() {
	return head == tail;
}
int isFull() {
	return head == (tail + 1) % MAX;	//MAX = 7のときhead =4、tail=3となる。head = tailではないのは、配列の要素を全て使うわけではないことが原因。
}										//（入る容量は要素-1。つまりMAX）

void enqueue(struct process x) {
	if (isFull()) {
		printf("これ以上入りません\n");
		return;
	}
	
	Q[tail] = x;
	if (tail + 1 == max) {
		tail = 0;
	}
	else {
		tail++;
	}
}

struct process dequeue() {
	struct process x;
	if (isEmpty()) {
		printf("これ以上取り除けません\n");

	}
	x = Q[head];
	if (head + 1 == max) {
		head = 0;
	}
	else {
		head++;
	}
	return x;
}


int main() {
	int i, n, q, total = 0;
	char nameAndTime[20], nq[15], *cn;
	struct process p[50000];

	initialize();

	fgets(nq, sizeof(nq), stdin);
	cn = strtok(nq, " ");
	n = atoi(cn);
	cn = strtok(NULL, " ");
	q = atoi(cn);

	for (i = 0; i < n; i++) {
		fgets(nameAndTime, sizeof(nameAndTime), stdin);
		cn = strtok(nameAndTime, " ");
		strcpy(p[i].name, cn);
		cn = strtok(NULL, " ");
		p[i].time = atoi(cn);
		total += p[i].time;
		if (total > 1000000) {
			return 0;
		}
		enqueue(p[i]);
	}

	struct process x;
	int	cnt = 0, k = 0, sec = 0;

	while (cnt < n) {
		x = dequeue();
		if (x.time <= 0) {
			enqueue(x);
		}
		else {
			if (x.time - q <= 0) {
				sec += x.time;
				printf("%s %d\n", x.name, sec);
				x.time -= q;
				cnt++;
			}
			else {
				x.time -= q;
				sec += q;
				enqueue(x);
			}
			
		}
	}

}

//
