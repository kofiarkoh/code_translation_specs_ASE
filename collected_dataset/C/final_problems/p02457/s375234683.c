#include <stdio.h>
#include <stdlib.h>


#define	ll_int	long long int
#define	BUCKET_SIZE	1000000
#define	SUB_SIZE	1000


int ELEMENT_N;

typedef struct data_t{
	ll_int num;
	int n;
	struct data_t *next;
} DATA_T;

DATA_T **S;


// ハッシュテーブルの初期化
void Init()
{
	int i;

	for(i = 0; i < BUCKET_SIZE; i++)
		S[i] = NULL;
}


ll_int Hash(ll_int num)
{
	return num / SUB_SIZE;
}


DATA_T *Find(ll_int num)
{
	ll_int h;
	DATA_T *p;


	h = Hash(num);

	for(p = S[h]; p != NULL; p = p->next){
		if(num == p->num)
			return p;
	}

	return NULL;
}


void Insert(ll_int num)
{
	ll_int h;
	DATA_T *p;
	DATA_T *i, *j;


	if(( p = Find(num) )){
		p->n++;
		return;
	}


	/* 挿入処理(比較して該当する場所へ突っ込む) */
	if( ( p = (DATA_T *)malloc(sizeof(DATA_T)) ) == NULL){
		printf("malloc error\n");
		exit(1);
	}

	p->num = num;
	p->n = 1;
	h = Hash(num);

	if(S[h] == NULL){
	// 該当する添え字の箱が空の場合
		S[h] = p;
		p->next = NULL;
	}else if(num < S[h]->num){
	// 該当する添え字の先頭に挿入
		p->next = S[h];
		S[h] = p;
	}else{
	// 先頭以降への挿入
		for(i = S[h], j = i->next; j != NULL; i = j, j = j->next){
			if(i->num < num && num < j->num){
				i->next = p;
				p->next = j;
				break;
			}
		}

		if(j == NULL){
		// 最後尾に突っ込む
			i->next = p;
			p->next = j;
		}
	}

	ELEMENT_N++;
}


void Delete(ll_int num)
{
	DATA_T *p, *q;
	ll_int h;


	if(Find(num) == NULL)
		return;


	h = Hash(num);
	p = S[h];
	ELEMENT_N--;

	// 先頭にある場合の処理
	if(num == p->num){
		S[h] = p->next;
		free(p);
		return;
	}

	// 先頭以降の処理
	for(q = p->next;; p = q, q = q->next){
		if(num == q->num){
			p->next = q->next;
			free(q);
			break;
		}
	}
}


// lとrを基準にハッシュテーブルの添え字を計算、条件にあった数値を出力
void Dump(ll_int l, ll_int r)
{
	DATA_T *p;
	ll_int start_h, end_h;
	ll_int i;


	start_h = Hash(l);
	end_h = Hash(r);

	for(i = start_h; i <= end_h; i++){
		for(p = S[i]; p != NULL; p = p->next){
			if(r < p->num)
				return;

			if(l <= p->num && p->num <= r)
				printf("%lld\n", p->num);
		}
	}
}


int main()
{
	int q;
	int query;
	ll_int num, l, r;
	DATA_T *p;
	int i;


	ELEMENT_N = 0;
	S = (DATA_T **)malloc(sizeof(DATA_T *) * BUCKET_SIZE);

	scanf("%d", &q);

	for(i = 0; i < q; i++){
		scanf("%d", &query);

		if(query == 0){
		// insert
			scanf("%lld", &num);
			Insert(num);
			printf("%d\n", ELEMENT_N);
		}else if(query == 1){
		// find
			scanf("%lld", &num);
			if(( p = Find(num) ))
				printf("1\n");
			else
				printf("0\n");
		}else if(query == 2){
		// Delete
			scanf("%lld", &num);
			Delete(num);
		}else if(query == 3){
		// Dump
			scanf("%lld %lld", &l, &r);
			Dump(l, r);
		}
	}

	return 0;
}


/*
	1,ハッシュ関数でいつも使う「find」でDumpしようと思ったがタイムオーバー
	->失敗：いちいちfindで探索してると間に合わない

	2,ハッシュテーブルからデータを取り出してクイックソートからのDumpもタイムオーバー
	->失敗：いちいちソートしてるから間に合わない

	3,ハッシュテーブル自体を特殊な形にする
	->成功：ハッシュテーブルとほぼ同じだが箱の扱いを以下のようにする
	S[num] は「100 * num ～ 100 * num + 99」までの数値を入れる
	例：S[0]は0～99、S[100]は10000～10099という具合

	そしてInsertは数値の並びが昇順になるように処理

	これによって全ての操作が高速になる
*/
