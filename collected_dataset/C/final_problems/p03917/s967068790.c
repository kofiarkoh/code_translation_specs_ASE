#include <stdio.h>
#include <stdlib.h>
#define int long long
#define p (int)(1e9 + 7)

typedef struct {
	int N;
	int *u;
	int *u_rank;
}union_find;

union_find *make_union_find(int N){
	int i;
	union_find *uf = (union_find *)malloc(sizeof(union_find));
	uf->N = N;
	uf->u = (int *)malloc(sizeof(int) * N);
	uf->u_rank = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		uf->u[i] = i;
		uf->u_rank[i] = 1;
	}
	return uf;
}

int root_uf(int x, union_find *uf){
	int *u = uf->u;
	if(u[x] == x){
		return x;
	}
	else{
		u[x] = root_uf(u[x], uf);
		return u[x];
	}
}

void combine_uf(int x, int y, union_find *uf){
	int x_root = root_uf(x, uf);
	int y_root = root_uf(y, uf);
	int *u = uf->u;
	int *u_rank = uf->u_rank;
	if(x_root == y_root){
		return;
	}
	else if(u_rank[x_root] < u_rank[y_root]){
		u[x_root] = y_root;
		u_rank[y_root] += u_rank[x_root];
		u_rank[x_root] = 0;
	}
	else{
		u[y_root] = x_root;
		u_rank[x_root] += u_rank[y_root];
		u_rank[y_root] = 0;
	}
}

//xとyが同じ集合に属していれば1を,そうでなければ0を返す
int is_same_union_uf(int x, int y, union_find *uf){
	if(root_uf(x, uf) == root_uf(y, uf)){
		return 1;
	}
	else{
		return 0;
	}
}

//xが属する集合の要素数を返す
int rank_uf(int x, union_find *uf){
	return uf->u_rank[root_uf(x, uf)];
}

int MOD(int a){
	a %= p;
	return a >= 0 ? a : a + p;
}

signed compare(const void *a, const void *b){
	signed ac = (signed)(*(char *)a), bc = (signed)(*(char *)b);
//	printf("(a, b) = (%c, %c)\n", *(char *)a, *(char *)b);
//	printf("(ac, bc) = (%d, %d)\n", ac, bc);
	return ac - bc;
}

int judge(char s1, char s2, char s3, char s4){
	char ss[4] = {s1, s2, s3, s4};
/*	int i;
	for(i = 0; i < 4; i++){
		printf("%c ", ss[i]);
	}
	printf("\n");
*/	qsort(ss, 4, sizeof(char), compare);
/*	for(i = 0; i < 4; i++){
		printf("%c ", ss[i]);
	}
	printf("\n");
*/	if(ss[0] == ss[1]){
		if(ss[1] == ss[2]){
			if(ss[2] == ss[3]){
				return 4;
			}
			else{
				return 3;
			}
		}
		else{
			if(ss[2] == ss[3]){
				return 2;
			}
			else{
				return 1;
			}
		}
	}
	else if(ss[1] == ss[2]){
		if(ss[2] == ss[3]){
			return 3;
		}
		else{
			return 1;
		}
	}
	else if(ss[2] == ss[3]){
		return 1;
	}
	else{
		return 0;
	}
}

int power(int a, int N){
	if(N == 0){
		return 1;
	}
	else if(N % 2 == 0){
		return power(MOD(a * a), N / 2);
	}
	else{
		return MOD(a * power(a, N - 1));
	}
}

signed main(){
	int H, W, i, j;
	scanf("%lld%lld", &H, &W);
	char **S = (char **)malloc(sizeof(char *) * H);
	for(i = 0; i < H; i++){
		S[i] = (char *)malloc(sizeof(char) * (W + 1));
		scanf("%s", S[i]);
	}
	int h = H / 2, w = W / 2, type, ans = 1;
	union_find *uf = make_union_find(h + w);
	for(i = 0; i < h; i++){
		for(j = 0; j < w; j++){
			type = judge(S[i][j], S[H - 1 - i][j], S[i][W - 1 - j], S[H - 1 - i][W - 1 - j]);
//			printf("type(%lld, %lld) = %lld\n", i, j, type);
			if(type == 0){
				ans = MOD(ans * 12);
				combine_uf(i, h + j, uf);
			}
			else if(type == 1){
				ans = MOD(ans * 12);
			}
			else if(type == 2){
				ans = MOD(ans * 6);
			}
			else if(type == 3){
				ans = MOD(ans * 4);
			}
		}
	}
//	printf("ans = %lld\n", ans);
	for(i = 0; i < h + w; i++){
		if(root_uf(i, uf) == i){
			ans = MOD(ans * power(2, rank_uf(i, uf) - 1));
		}
	}
	if(H % 2 == 1){
		for(j = 0; j < w; j++){
			if(S[H / 2][j] != S[H / 2][W - 1 - j]){
				ans = MOD(ans * 2);
				break;
			}
		}
	}
	if(W % 2 == 1){
		for(i = 0; i < h; i++){
			if(S[i][W / 2] != S[H - 1 - i][W / 2]){
				ans = MOD(ans * 2);
				break;
			}
		}
	}
	printf("%lld\n", ans);
	return 0;
}