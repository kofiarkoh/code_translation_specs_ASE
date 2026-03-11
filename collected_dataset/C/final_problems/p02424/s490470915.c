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

void swap(int* a, int* b)
{
	int tmp;
	tmp = *a;
	*a = *b;
	*b = tmp;
}

int cnt = 0;


void insertion_sort(int *a, int n,int g)
{
	int i, j;
	int k;

	for (i = g; i < n; i++)
	{
		k = i;

		while (a[k] < a[k - g] && k>=g)
		{
			swap(&a[k], &a[k - g]);
			k-=g;
			cnt++;
		}
	}
}

/*
void insertion_sort(int *A, int n, int g) {
	int i, j, v;
	for (i = g; i < n; i++) {
		v = A[i];
		j = i - g;
		while (j >= 0 && A[j] > v) {
			A[j + g] = A[j];
			j = j - g;
			cnt++;
		}
		A[j + g] = v;
	}
}*/


void shell_sort(int *a, int n)
{
	long long int h=1;
	int m = 1;
	int G[100];
	int i, j;

	while ((h*3+1) < n)
	{
		h = h * 3 + 1;
		m++;
	}

	for (i = 0; i < m; i++)
	{
		G[i] = h;
		h /= 3;
	}

	printf("%d\n", m);

	for (i = 0; i < m; i++)
	{
		if (i == m - 1)
			printf("%d\n", G[i]);
		else
			printf("%d ", G[i]);
	}

	for (i = 0; i < m; i++)
		insertion_sort(a, n, G[i]);

	printf("%d\n", cnt);
}
/*

int GCD(int x, int y)
{
	if (x < y)
		swap(&x, &y);

	int z;

	while (y > 0)
	{
		z = x;
		x = y;
		y = z % y;
	}
	return x;
}

int prime(int x)
{
	int i, j;

	for (i = 2; i <= sqrt(x); i++)
	{
		if (x % i == 0)
			return -1;
	}

	return 1;
}

void bubble_sort(int* p, int n)
{
	int i, j;

	for (i = 0; i < n; i++)
	{
		for (j=n-1; j > i ; j--)
		{
			if (p[j] < p[j - 1])
			{
				swap(&p[j], &p[j - 1]);
				cnt++;
			}
		}
	}
}
*/

/*
void selection_sort(int* p, int n)
{
	int i, j, min;

	for (i = 0; i < n; i++)
	{
		min = i;
		for (j = i+1; j < n; j++)
		{
			if (p[j] < p[min])
				min = j;
		}
		if (min != i)
		{
			swap(&p[min], &p[i]);
			cnt++;
		}
	}
}*/


int binary_search(int* a, int n,int key)
{
	int low, middle, high;

	low = 0, high = n - 1;

	while (low <= high)
	{
		middle = (low + high) / 2;

		if (key < a[middle])
			high = middle - 1;
		else if (key > a[middle])
			low = middle + 1;
		else
			return 1;
	}
	return 0;
}

void reverse(int a[], int b, int e)
{
	int i;

	for (i = 0; i <= (e-b-1) / 2; i++)
	{
		swap(&a[b+i], &a[e - 1 - i]);
	}
}

int lower_bound(int a[], int n, int key)
{
	int low, high, middle;

	low = 0, high = n - 1;

	int i;

	if (key > a[n-1])
		return n;

	if (key < a[0])
		return 0;

	while (low <= high)
	{
		middle = (low + high) / 2;

		if (a[middle] == key)
		{
			for (i = middle; i >= 0; i--)
			{
				if (a[i] != key)
					return i + 1;
			}
			return 0;
		}
		else if (key < a[middle])
			high = middle - 1;
		else if (key > a[middle])
			low = middle + 1;
	}

	if (a[middle] < key)
	{
		for (i = middle; i < n; i++)
		{
			if (a[i] >= key)
				return i;
		}
		return n;
	}
	else
	{
		for (i = middle; i >= 0; i--)
		{
			if (a[i] < key)
				return i + 1;
		}
		return 0;
	}
}

int upper_bound(int a[], int n, int key)
{
	int low, high, middle;

	low = 0, high = n - 1;

	int i;

	if (key > a[n - 1])
		return n;

	if (key < a[0])
		return 0;

	while (low <= high)
	{
		middle = (low + high) / 2;

		if (a[middle] == key)
		{
			for (i = middle; i < n; i++)
			{
				if (a[i] != key)
					return i;
			}
			return n;
		}
		else if (key < a[middle])
			high = middle - 1;
		else if (key > a[middle])
			low = middle + 1;
	}

	if (a[middle] < key)
	{
		for (i = middle; i < n; i++)
		{
			if (a[i] >= key)
				return i;
		}
		return n;
	}
	else
	{
		for (i = middle; i >= 0; i--)
		{
			if (a[i] < key)
				return i + 1;
		}
		return 0;
	}
}

void print_binary(long long int x)
{
	long long int mask = (long long int)1 << 32;

	while (mask >>= 1)
	{
		if (mask & x)
			printf("1");
		else
			printf("0");
	}
	printf("\n");
}

int main()
{
	long long int a, b;
	long long int x, y, z;

	scanf("%lld%lld", &a, &b);

	x = a & b;

	print_binary(x);

	y = a | b;

	print_binary(y);

	z = a ^ b;

	print_binary(z);

	return 0;
}
