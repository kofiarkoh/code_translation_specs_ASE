#include <stdio.h>
#include <stdlib.h>

#define N 100000

typedef struct list
{
	int key;
	struct list *prev;
	struct list *next;
} list;

list head;
list tail;

void CreateList(int k)
{
	list *x;
	x = (list*)malloc(sizeof(list));
	
	x->next = head.next;
	head.next->prev = x;
	x->prev = &head;
	head.next = x;
	x->key = k;
}

void Insert(list *x)
{
	list *com;
	com = &head;
	
	while(1)
	{
		if(com == tail.prev)
		{
			x->next = &tail;
			tail.prev->next = x;
			x->prev = tail.prev;
			tail.prev = x;
			break;
		}
		if(x->key < com->next->key)
		{
			x->next = com->next;
			com->next->prev = x;
			x->prev = com;
			com->next = x;
			break;
		}
		com = com->next;
	}
}

void DeleteFirst()
{
	head.next->next->prev = head.next->prev;
	head.next = head.next->next;
}

void InsertionSort(int A[], int n, int g)
{
	int i, j;
	int tmp;
	
	for(i = g; i < n; i++)
	{
		j = i - g;
		tmp = A[i];
		while(j >= 0 && A[j] > tmp)
		{
			A[j + g] = A[j];
			j = j - g;
			A[j + g] = tmp;
		}
	}
}	
int CreateG(int n, int d, int G[])
{
	int i;
	static int m = 0;
	static int factorial = 1;
	
	d += factorial;
	factorial *= 3;
	
	if(d > n) return 0;
	CreateG(n, d, G);
	G[m] = d;
	m++;
	
	return m;
}
void BubbleSort(int n, int *nl)
{
	int i;
	int m;
	int G[100];
	
	m = CreateG(n, 0, G);
	for(i = 0; i < m; i++)
	{
		InsertionSort(nl, n, G[i]);
	}
}

int main()
{
	int i;
	int n, k;
	int nl[N];
	list *x;
	
	scanf("%d %d", &n, &k);
	for(i = 0; i < n; i++)
	{
		scanf("%d", &nl[i]);
	}
	BubbleSort(n, nl);
	
	head.next = &tail;
	tail.prev = &head;
	for(i = 0; i < k; i++)
	{
		CreateList(0);
	}
	for(i = 0; i < n; i++)
	{
		head.next->key += nl[n - i - 1];
		x = head.next;
		DeleteFirst();
		Insert(x);
	}
	printf("%d\n", tail.prev->key);
	
	return 0;
}	