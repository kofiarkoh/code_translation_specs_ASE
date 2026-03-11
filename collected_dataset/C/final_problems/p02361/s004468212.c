#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>
#include <limits.h>
#define maxN 50
#define INF INT_MAX
#define WHITE 0
#define GRAY 1
#define BLACK 2


typedef struct cell
{
	int key; int vertex;
}cell;

int parent(int i) { return (i + 1) / 2; }
int left(int i) { return i * 2 + 1; }
int right(int i) { return i * 2 + 2; }
void swap(int* x, int* y)
{
	int t = *x;
	*x = *y;
	*y = t;
}

void swapcell(cell* x, cell* y)
{
	cell t = *x;
	*x = *y;
	*y = t;
}


void downheap_sort(cell* H, int *adr, int i)
{
	int u = 0;
	while (1)
	{
		int l = left(u);
		int r = right(u);
		int s = u;
		if (l <= i && H[l].key < H[u].key)s = l;
		if (r <= i && H[r].key < H[s].key)s = r;
		if (s == u)break;
		swapcell(&H[u], &H[parent(u)]);
		swap(&adr[H[u].vertex], &adr[H[parent(u)].vertex]);
		u = s;
	}
}
void decrease_key(struct cell *H, int * adr, int i, int a)
{
	H[i].key = a;
	downheap_sort(H, adr, i);
}

int delete_min(cell *H, int *adr, int hsize)
{
	int a = H[0].key;
	int v = H[0].vertex;
	adr[v] = -1;
	if (hsize > 1)
	{
		swapcell(&H[0], &H[hsize - 1]);
		adr[H[0].vertex] = 0;
		downheap_sort(H, adr, hsize - 2);
	}
	return v;
}
void insert(cell* H, int *adr, int hsize, int a, int v)
{
	H[hsize - 1].key = a;
	H[hsize - 1].vertex = v;
	adr[v] = hsize - 1;
	downheap_sort(H, adr, hsize - 1);
}
int main(void)
{
	int N, M;
	int Lmat[maxN][maxN];
	int u, v, len, w, i, j;
	int v0, hsize;
	int d[maxN], p[maxN], adr[maxN];
	cell Heap[maxN];
	fscanf(stdin, "%d%d%d", &N, &M, &v0);
	for (i = 0; i < N; i++)
	{
		for (j = 0; j < N; j++)
		{
			Lmat[i][j] = INF;
		}
	}

	for (i = 0; i < M; i++)
	{
		fscanf(stdin, "%d%d%d", &u, &v, &len);
		Lmat[u][v] = len;
	}



	for (i = 0; i < N; i++)
	{
		d[i] = INF;
		p[i] = -1;
		adr[i] = -1;

	}
	d[v0] = 0;
	hsize = 1;
	insert(Heap, adr, hsize, d[v0], v0);

	while (hsize != 0)
	{
		v = delete_min(Heap, adr, hsize);
		hsize--;
		for (w = 0; w < N; w++)
		{
			if (Lmat[v][w] == INF)continue;
			if (d[w] == INF)
			{
				d[w] = d[v] + Lmat[v][w];
				p[w] = v;
				hsize++;
				insert(Heap, adr, hsize, d[w], w);
			}
			else if (d[w] > d[v] + Lmat[v][w])
			{
				d[w] = d[v] + Lmat[v][w];
				p[w] = v;
				decrease_key(Heap, adr, adr[w], d[w]);
			}
		}
	}
	


	for (i = 0; i < N; i++)
	{
		(d[i]==INF)?printf("INF\n"):printf("%d\n",d[i]);
	}
	return 0;
}