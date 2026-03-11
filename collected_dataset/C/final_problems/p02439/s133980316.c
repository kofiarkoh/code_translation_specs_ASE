//#pragma comment (linker,"/STACK:256000000")
#define _USE_MATH_DEFINES
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define next(a) ( ((a)+1)%SIZE )
#define ABS(x) ( (x)>0 ? x : -(x) )
#define MIN(x,y) ( (x) < (y) ? (x) : (y) )
#define MAX(x,y) ( (x) > (y) ? (x) : (y) )
#define INF 1000000000
#define SIZE 100000

#define INF 1000000000
#define LLINF 10000000000000000

typedef long long ll;

typedef struct queue
{
	int front;
	int rear;
	int *data;
}queue;

/*
void init(queue* p)
{
	p->front = 0;
	p->rear = 0;
}

void enqueue(queue *p,int x)
{
	if (next(p->rear) == p->front)
		printf("OVERFLOW!\n");
	p->data[p->rear] = x;
	p->rear = next(p->rear);
}

int dequeue(queue *p)
{
	int x;

	if (p->front == p->rear)
		printf("NO DADA\n");

	x = p->data[p->front];
	p->front = next(p->front);

	return x;
}

int empty(queue *p)
{
	return p->front == p->rear;
}

void print(char* str, int a, int b)
{
	int i;

	for (i = a; i <= b; i++)
		printf("%c", str[i]);
	printf("\n");
}
*/

void reverse(char* str, int a, int b)
{
	int i,j=0;
	char t;

	for (i = a; i <= (a + b) / 2; i++)
	{
		t = str[i];
		str[i] = str[b - j];
		str[b - j] = t;
		j++;
	}
}

void replace(char* str, int a, int b, char* temp)
{
	int i, j=0;

	for (i = a; i <= b; i++)
	{
		str[i] = temp[j];
		j++;
	}
}

double rad(int C)
{
	return M_PI * C / 180;
}

double distance(int* a, int* b, int n, int p)
{
	double sum = 0.0;

	int i, j;

	int max = 0;

	for (i = 0; i < n; i++)
	{
		sum += pow(ABS(a[i] - b[i]), p);

		if (ABS(a[i] - b[i]) > max)
			max = ABS(a[i] - b[i]);
	}

	if (p == INF)
		return max;

	return pow(sum, (double)1 / p);
}

/*
typedef struct Vector
{
	int data[10];
	int size;
	int max;
}Vector;

Vector *init()
{
	Vector* p;

	//p = malloc(sizeof(int)*(10+2));
	p = (Vector*)(sizeof(int) * 12);
	//p->data = (int*)malloc(sizeof(int) * 10);
	p->size = 0;
	p->max = 10;

	return p;
}

Vector* Extend(Vector* p)
{
	Vector* q;

	q = (Vector*)malloc(sizeof(int) * p->max * 2);

	memcpy(q, p, sizeof(int) * p->size);

	p->max *= 2;

	free(p);

	return q;
}

Vector *pushBack(Vector *p,int x)
{
	if (p->size == p->max)
		p = Extend(p);

	p->data[p->size++] = 1;

	return p;
}

int randomAccess(Vector* p,int x)
{
	return p->data[x];
}

Vector* popBack(Vector* p)
{
	//free(p->data[--(p->size)]);
	--p->size;
	return p;
}
*/


int main()
{
	ll a,b,c;
	ll max = -LLINF, min = LLINF;

	scanf("%lld%lld%lld", &a, &b, &c);

	if (a > max)
		max = a;
	if (b > max)
		max = b;
	if (c > max)
		max = c;

	if (a < min)
		min = a;
	if (b < min)
		min = b;
	if (c < min)
		min = c;

	printf("%lld %lld\n", min, max);

	return 0;
}

