#include <stdio.h>
#include <stdlib.h>

typedef enum {
  false,
  true
}boolean;

typedef struct {
	int a;
	int b;
	int c;
}edge;

typedef struct vsub{
	int v;
	int num;
	struct vsub **nexts;
	int *c;
}vertex;

boolean search(int n, vertex *ver, int sum, int dest, int border){
	if(sum >= border || n == 0){
		return false;
	}
	if(sum < border && ver->v == dest){
		return true;
	}
	int i;
	for(i = 0; i < ver->num ;i++){
		if(search(n - 1 , (ver->nexts)[i], sum + (ver->c)[i], dest, border) == true){
			return true;
		}
	}
	return false;
}

int main(){
	int N, M, i, a, b, c, ans = 0;
	scanf("%d%d", &N, &M);
	edge E[M];
	vertex *V[N];
	for(i = 0; i < N; i++){
		V[i] = (vertex *)malloc(sizeof(vertex));
		V[i]->v = i;
		V[i]->num = 0;
		V[i]->nexts = (vertex **)malloc(sizeof(vertex *) * N);
		V[i]->c = (int *)malloc(sizeof(int) * N);
	}
	for(i = 0; i < M; i++){
		scanf("%d%d%d", &a, &b, &c);
		a--;
		b--;
		E[i].a = a;
		E[i].b = b;
		E[i].c = c;
		(V[a]->nexts)[V[a]->num] = V[b];
		(V[a]->c)[V[a]->num] = c;
		(V[a]->num)++;
		(V[b]->nexts)[V[b]->num] = V[a];
		(V[b]->c)[V[b]->num] = c;
		(V[b]->num)++;
	}
	for(i = 0; i < M; i++){
		if(search(N, V[E[i].a], 0, E[i].b, E[i].c) == true){
			ans++;
		}
	}
	printf("%d\n", ans);
	return 0;
}