#include <stdio.h>
#include <stdlib.h>
#define inf (int)(1e9)

int N, M, *a, *b, **getA, **getB;

typedef struct {
	int i;
	int j;
	int k;
	int l;
	int pass;
}pair4;

int max(int a, int b){
	return a >= b ? a : b;
}

int min(int a, int b){
	return a <= b ? a : b;
}

#define AVL_keytype pair4
#define AVL_valtype int


//static int malloc_cont;
//static int free_cont;

typedef struct AVL_node_sub{
	AVL_keytype key; //添え字
	AVL_valtype val; //値
	int ele_num; //木に含まれる要素数
	int height; //木の高さ
	struct AVL_node_sub *left; //左の子へのポインタ
	struct AVL_node_sub *right; //右の子へのポインタ
}AVL_node;

typedef struct {
	AVL_node *root;
}AVL_tree;

AVL_tree *tA, *tB;

//比較関数
//a < b なら負の値
//a = b なら0
//a > b なら正の値
int compare_AVL(AVL_keytype a, AVL_keytype b){
	if(a.i != b.i){
		return a.i - b.i;
	}
	else if(a.j != b.j){
		return a.j - b.j;
	}
	else if(a.k != b.k){
		return a.k - b.k;
	}
	else{
		return a.l - b.l;
	}
}

int ele_num(AVL_node *r){
	if(r == NULL){
		return 0;
	}
	else{
		return r->ele_num;
	}
}

int height(AVL_node *r){
	if(r == NULL){
		return 0;
	}
	else{
		return r->height;
	}
}

//tの指すノードを開放する
//AVL_valtypeなどがポインタ型の時はそれもfreeする
void release_AVL_node(AVL_node *r){
	free(r);
//	free_cont++;
}

AVL_node *build_AVL_node(AVL_keytype key, AVL_valtype val, AVL_node *left, AVL_node *right){
	AVL_node *newr;
	int left_h = height(left);
	int right_h = height(right);
	if(left_h > right_h + 1){
		AVL_node *ll = left->left;
		AVL_node *lr = left->right;
		if(height(ll) < height(lr)){
			newr = build_AVL_node(lr->key, lr->val, build_AVL_node(left->key, left->val, ll, lr->left), build_AVL_node(key, val, lr->right, right));
			release_AVL_node(lr);
		}
		else{
			newr = build_AVL_node(left->key, left->val, ll, build_AVL_node(key, val, lr, right));
		}
		release_AVL_node(left);
	}
	else if(right_h > left_h + 1){
		AVL_node *rr = right->right;
		AVL_node *rl = right->left;
		if(height(rr) < height(rl)){
			newr = build_AVL_node(rl->key, rl->val, build_AVL_node(key, val, left, rl->left), build_AVL_node(right->key, right->val, rl->right, rr));
			release_AVL_node(rl);
		}
		else{
			newr = build_AVL_node(right->key, right->val, build_AVL_node(key, val, left, rl), rr);
		}
		release_AVL_node(right);
	}
	else{
//		malloc_cont++;
		newr = (AVL_node *)malloc(sizeof(AVL_node));
		newr->key = key;
		newr->val = val;
		newr->ele_num = ele_num(left) + ele_num(right) + 1;
		newr->height = max(left_h, right_h) + 1;
		newr->left = left;
		newr->right = right;
	}
	return newr;
}

AVL_node *find_AVL_sub(AVL_keytype key, AVL_node *r){
	if(r == NULL){
		return NULL;
	}
	int comp = compare_AVL(key, r->key);
	if(comp == 0){
		return r;
	}
	else if(comp < 0){
		return find_AVL_sub(key, r->left);
	}
	else{
		return find_AVL_sub(key, r->right);
	}
}

AVL_node *kth_smallest_AVL_sub(int k, AVL_node *r){
	if(r == NULL || k < 1){
		printf("In function 'kth_smallest_AVL_sub':\nargument 'k' is out of range\n");
		return NULL;
	}
	else if(r->ele_num < k){
		printf("In function 'kth_smallest_AVL_sub':\nargument 'k' is out of range\n");
		return NULL;
	}
	else if(ele_num(r->left) == k - 1){
		return r;
	}
	else if(ele_num(r->left) > k - 1){
		return kth_smallest_AVL_sub(k, r->left);
	}
	else{
		return kth_smallest_AVL_sub(k - ele_num(r->left) - 1, r->right);
	}
}

int num_less_than_AVL_sub(AVL_keytype key, AVL_node *r){
	if(r == NULL){
		return 0;
	}
	else if(compare_AVL(key, r->key) < 0){
		return num_less_than_AVL_sub(key, r->left);
	}
	else{
		return ele_num(r->left) + num_less_than_AVL_sub(key, r->right) + 1;
	}
}

AVL_node *next_largest_AVL_sub(AVL_keytype key, AVL_node *r){
	if(r == NULL){
		return NULL;
	}
	else if(compare_AVL(key, r->key) <= 0){
		return next_largest_AVL_sub(key, r->left);
	}
	else{
		AVL_node *candidate = next_largest_AVL_sub(key, r->right);
		if(candidate == NULL){
			return r;
		}
		else{
			return candidate;
		}
	}
}

AVL_node *next_smallest_AVL_sub(AVL_keytype key, AVL_node *r){
	if(r == NULL){
		return NULL;
	}
	else if(compare_AVL(key, r->key) >= 0){
		return next_smallest_AVL_sub(key, r->right);
	}
	else{
		AVL_node *candidate = next_smallest_AVL_sub(key, r->left);
		if(candidate == NULL){
			return r;
		}
		else{
			return candidate;
		}
	}
}

AVL_node *insert_AVL_sub(AVL_keytype key, AVL_valtype val, AVL_node *r){
	AVL_node *newr;
	if(r == NULL){
		newr = build_AVL_node(key, val, NULL, NULL);
	}
	else{
		int comp = compare_AVL(key, r->key);
		if(comp == 0){
			printf("In function 'insert_AVL_sub':\nkey '%d' already exists\n", key);
			newr = build_AVL_node(r->key, val, r->left, r->right);
		}
		else if(comp < 0){
			newr = build_AVL_node(r->key, r->val, insert_AVL_sub(key, val, r->left), r->right);
		}
		else{
			newr = build_AVL_node(r->key, r->val, r->left, insert_AVL_sub(key, val, r->right));
		}
		release_AVL_node(r);
	}
	return newr;
}

AVL_node *erase_AVL_sub_sub(AVL_node *r){
	if(r->left == NULL){
		return r->right;
	}
	else{
		return build_AVL_node(r->key, r->val, erase_AVL_sub_sub(r->left), r->right);
	}
}

AVL_node *erase_AVL_sub(AVL_keytype key, AVL_node *r, AVL_node **ans){
	AVL_node *newr;
	if(r == NULL){
		newr = NULL;
	}
	else{
		int comp = compare_AVL(key, r->key);
		if(comp == 0){
			*ans = r;
			if(r->left == NULL && r->right == NULL){
				newr = NULL;
			}
			else if(r->right == NULL){
				newr = r->left;
			}
			else if(r->left == NULL){
				newr = r->right;
			}
			else{
				AVL_node *next_larger = kth_smallest_AVL_sub(1, r->right);
				newr = build_AVL_node(next_larger->key, next_larger->val, r->left, erase_AVL_sub_sub(r->right));
				release_AVL_node(next_larger);
			}
		}
		else{
			if(comp < 0){
				newr = build_AVL_node(r->key, r->val, erase_AVL_sub(key, r->left, ans), r->right);
			}
			else{
				newr = build_AVL_node(r->key, r->val, r->left, erase_AVL_sub(key, r->right, ans));
			}
			release_AVL_node(r);
		}
	}
	return newr;
}

void storeall_AVL_sub(AVL_keytype *array, int k, AVL_node *r){
	if(r != NULL){
		storeall_AVL_sub(array, k, r->left);
		array[k + ele_num(r->left)] = r->key;
		storeall_AVL_sub(array, k + ele_num(r->left) + 1, r->right);
	}
}

void outall_AVL_sub(AVL_node *r){
	if(r != NULL){
		outall_AVL_sub(r->left);
		printf("(key, val, ele_num, height) = (%d, %d, %d, %d)\n", r->key, r->val, r->ele_num, r->height);
		outall_AVL_sub(r->right);
	}
}

//AVL_treeを生成する
AVL_tree *make_AVL_tree(){
	AVL_tree *t = (AVL_tree *)malloc(sizeof(AVL_tree));
	t->root = NULL;
	return t;
}

//tに含まれるノードの数を返す
int element_num_AVL(AVL_tree *t){
	return ele_num(t->root);
}

//添え字がkeyのノードへのポインタを返す
//なければNULLを返す
AVL_node *find_AVL(AVL_keytype key, AVL_tree *t){
	return find_AVL_sub(key, t->root);
}

//小さい順にk番目のkeyのノードへのポインタを返す
//1 ≦ k ≦ ele_num(t->root) を満たさない場合はNULLを返す(メッセージが出る)
AVL_node *kth_smallest_AVL(int k, AVL_tree *t){
	return kth_smallest_AVL_sub(k, t->root);
}

//添え字がkey以下のノードの数を返す
int num_less_than_AVL(AVL_keytype key, AVL_tree *t){
	return num_less_than_AVL_sub(key, t->root);
}

//keyよりも小さい中で最大の添え字のノードへのポインタを返す
//なければNULLを返す
AVL_node *next_largest_AVL(AVL_keytype key, AVL_tree *t){
	return next_largest_AVL_sub(key, t->root);
}

//keyよりも大きい中で最小の添え字のノードへのポインタを返す
//なければNULLを返す
AVL_node *next_smallest_AVL(AVL_keytype key, AVL_tree *t){
	return next_smallest_AVL_sub(key, t->root);
}

//添え字key, 値valのノードを挿入する
//既に存在する場合は値が上書きされる(メッセージが出る)
void insert_AVL(AVL_keytype key, AVL_valtype val, AVL_tree *t){
	t->root = insert_AVL_sub(key, val, t->root);
}

//添え字keyのノードを削除しそれを返す
//存在しない場合はNULLを返す
AVL_node *erase_AVL(AVL_keytype key, AVL_tree *t){
	AVL_node *ans = NULL;
	t->root = erase_AVL_sub(key, t->root, &ans);
	return ans;
}

//全ノードのkeyを小さい順に格納した配列を返す
AVL_keytype *storeall_AVL(AVL_tree *t){
	AVL_keytype *array = (AVL_keytype *)malloc(sizeof(AVL_keytype) * element_num_AVL(t));
	storeall_AVL_sub(array, 0, t->root);
	return array;
}

//全ノードの中身をkeyの小さい順に出力する
void outall_AVL(AVL_tree *t){
	outall_AVL_sub(t->root);
}

int getAf(int l, int r){
	if(l <= r){
		return getA[l][r];
	}
	else{
		return 0;
	}
}

int getBf(int l, int r){
	if(l <= r){
		return getB[l][r];
	}
	else{
		return 0;
	}
}

int dpB(int i, int j, int k, int l, int pass);

int dpA(int i, int j, int k, int l, int pass){
//	printf("dpA(%d, %d, %d, %d, %d)\n", i, j, k, l, pass);
	int ans;
	if(pass >= 2){
		return 0;
	}
	if(i == N && j == M){
		return getAf(i - k, i - 1) - getBf(j - l, j - 1);
	}
	pair4 p4;
	p4.i = i;
	p4.j = j;
	p4.k = k;
	p4.l = l;
	p4.pass = pass;
	AVL_node *r = find_AVL(p4, tA);
	if(r != NULL){
		ans =  r->val;
	}
	else{
		if(i < N){
			if(a[i] >= 0){
				ans = max(dpB(i + 1, j, k + 1, l, 0), dpB(i, j, 0, 0, pass + 1) + getAf(i - k, i - 1) - getBf(j - l, j - 1));
			}
			else{
				ans = max(dpB(i + 1, j, k + 1, 0, 0), dpB(i, j, 0, 0, pass + 1) + getAf(i - k, i - 1) - getBf(j - l, j - 1));
			}
		}
		else{
			ans = dpB(i, j, 0, 0, pass + 1) + getAf(i - k, i - 1) - getBf(j - l, j - 1);
		}
		insert_AVL(p4, ans, tA);
	}
//	printf("dpA(%d, %d, %d, %d, %d) = %d\n", i, j, k, l, pass, ans);
	return ans;
}

int dpB(int i, int j, int k, int l, int pass){
//	printf("dpB(%d, %d, %d, %d, %d)\n", i, j, k, l, pass);
	int ans;
	if(pass >= 2){
		return 0;
	}
	if(i == N && j == M){
		return getAf(i - k, i - 1) - getBf(j - l, j - 1);
	}
	pair4 p4;
	p4.i = i;
	p4.j = j;
	p4.k = k;
	p4.l = l;
	p4.pass = pass;
	AVL_node *r = find_AVL(p4, tB);
	if(r != NULL){
		ans = r->val;
	}
	else{
		if(j < M){
			if(b[j] >= 0){
				ans = min(dpA(i, j + 1, k, l + 1, 0), dpA(i, j, 0, 0, pass + 1) + getAf(i - k, i - 1) - getBf(j - l, j - 1));
			}
			else{
				ans = min(dpA(i, j + 1, 0, l + 1, 0), dpA(i, j, 0, 0, pass + 1) + getAf(i - k, i - 1) - getBf(j - l, j - 1));
			}
		}
		else{
			ans = dpA(i, j, 0, 0, pass + 1) + getAf(i - k, i - 1) - getBf(j - l, j - 1);
		}
		insert_AVL(p4, ans, tB);
		return ans;
	}
//	printf("dpB(%d, %d, %d, %d, %d) = %d\n", i, j, k, l, pass, ans);
	return ans;
}

int main(){
	int i, j;
	scanf("%d%d", &N, &M);
	a = (int *)malloc(sizeof(int) * N);
	b = (int *)malloc(sizeof(int) * M);
	for(i = 0; i < N; i++){
		scanf("%d", &a[i]);
	}
	for(j = 0; j < M; j++){
		scanf("%d", &b[j]);
	}
	getA = (int **)malloc(sizeof(int *) * N);
	for(i = 0; i < N; i++){
		getA[i] = (int *)malloc(sizeof(int) * N);
		getA[i][i] = max(0, a[i]);
		for(j = i + 1; j < N; j++){
			getA[i][j] = getA[i][j - 1] + max(0, a[j]);
		}
	}
	getB = (int **)malloc(sizeof(int *) * M);
	for(i = 0; i < M; i++){
		getB[i] = (int *)malloc(sizeof(int) * M);
		getB[i][i] = max(0, b[i]);
		for(j = i + 1; j < M; j++){
			getB[i][j] = getB[i][j - 1] + max(0, b[j]);
		}
	}
	tA = make_AVL_tree();
	tB = make_AVL_tree();
	printf("%d\n", dpA(0, 0, 0, 0, 0));
	return 0;
}
