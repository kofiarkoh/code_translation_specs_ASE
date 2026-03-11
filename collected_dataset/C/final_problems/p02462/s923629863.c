#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>


#define	ll_int	long long int
#define	BUCKET_SIZE	500000	// 26^4 = 456,976


// keyの重複を可能にするための構造体(数値リスト)
typedef struct sub_t{
	ll_int x;		// 入力した数値
	struct sub_t *next;	// 次の数値へのポインタ
} SUB_T;


// 似非ハッシュテーブルの構造体
typedef struct data_t{
	char key[21];		// 入力されたkeyをそのまま記録
	SUB_T *last_in;		// 最後に数値リストへ追加したデータへのポインタ
	SUB_T *sub;		// 数値リストへのポインタ(同じkeyをもつ数値同士を集めたリスト)
	struct data_t *next;	// 次のkeyへのポインタ
} DATA_T;

DATA_T **S;


// ハッシュテーブルの初期化
void Init()
{
	int i;

	for(i = 0; i < BUCKET_SIZE; i++)
		S[i] = NULL;
}


// 似非ハッシュ値の算出
ll_int Hash(char *key)
{
	ll_int sum = 0;
	int i;

	for(i = 0; i < 4 && i < strlen(key); i++)
		sum += (key[i] - 97) * pow(26, 3 - i);

	return sum;
}


// 受け取ったkeyがあるかどうか探索する
// 見つかればそのkeyへのアドレス、見つからなければNULLを返す
DATA_T *Get(char *key)
{
	ll_int h;
	DATA_T *p;

	h = Hash(key);

	for(p = S[h]; p != NULL; p = p->next){
		if(!strcmp(key, p->key))
			return p;
	}

	return NULL;
}


void Insert(char *key, ll_int x)
{
	ll_int h;
	DATA_T *p;
	SUB_T *sub_p;
	DATA_T *i, *j;

	if( ( sub_p = (SUB_T *)malloc(sizeof(SUB_T)) ) == NULL){
		printf("malloc error\n");
		exit(1);
	}
	sub_p->x= x;
	sub_p->next = NULL;

	if(( p = Get(key) )){
	// keyが重複したら連結リストへ追加
		sub_p->x = x;
		sub_p->next = NULL;
		p->last_in->next = sub_p;
		p->last_in = sub_p;
		return;
	}


	/* 挿入処理(比較して該当する場所へ突っ込む) */
	if( ( p = (DATA_T *)malloc(sizeof(DATA_T)) ) == NULL){
		printf("malloc error\n");
		exit(1);
	}
	strcpy(p->key, key);
	p->last_in = sub_p;
	p->sub = sub_p;
	h = Hash(key);

	if(S[h] == NULL){
	// 該当する添え字の箱が空の場合
		S[h] = p;
		p->next = NULL;
	}else if(strcmp(key, S[h]->key) < 0){
	// 該当する添え字の先頭に挿入
		p->next = S[h];
		S[h] = p;
	}else{
	// 先頭以降への挿入
		for(i = S[h], j = i->next; j != NULL; i = j, j = j->next){
			if(strcmp(i->key, key) < 0 && strcmp(key, j->key) < 0){
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
}


void Delete(char *key)
{
	DATA_T *p, *q;
	SUB_T *sub_p, *sub_q;
	ll_int h;

	if(Get(key) == NULL)
		return;


	h = Hash(key);
	p = S[h];

	// 先頭にある場合の処理
	if(!strcmp(key, p->key)){
		S[h] = p->next;

		// 連結リストの削除
		for(sub_p = p->sub; sub_p != NULL; sub_p = sub_q){
			sub_q = sub_p->next;
			free(sub_p);
		}

		free(p);
		return;
	}

	// 先頭以降の処理
	for(q = p->next;; p = q, q = q->next){
		if(!strcmp(key, q->key)){
			p->next = q->next;

			// 連結リストの削除
			for(sub_p = q->sub; sub_p != NULL; sub_p = sub_q){
				sub_q = sub_p->next;
				free(sub_p);
			}

			free(q);
			break;
		}
	}
}


// lとrを基準にハッシュテーブルの添え字を計算、条件にあった数値を出力
void Dump(char *l, char *r)
{
	DATA_T *p;
	SUB_T *sub_p;
	ll_int start_h, end_h;
	ll_int i;

	start_h = Hash(l);
	end_h = Hash(r);

	for(i = start_h; i <= end_h; i++){
		for(p = S[i]; p != NULL; p = p->next){
			if(strcmp(r, p->key) < 0)
				return;

			if(strcmp(l, p->key) <= 0 && strcmp(p->key, r) <= 0){
				for(sub_p = p->sub; sub_p != NULL; sub_p = sub_p->next)
					printf("%s %lld\n", p->key, sub_p->x);
			}
		}
	}
}


int main()
{
	int q, query;
	char in_str[50];
	char key[21], x[21];
	DATA_T *p;
	SUB_T *sub_p;
	int i;

	S = (DATA_T **)malloc(sizeof(DATA_T *) * BUCKET_SIZE);

	Init();

	fgets(in_str, sizeof(in_str), stdin);
	sscanf(in_str, "%d", &q);

	for(i = 0; i < q; i++){
		fgets(in_str, sizeof(in_str), stdin);
		in_str[strlen(in_str) - 1] = '\0';
		sscanf(in_str, "%d %s %s", &query, key, x);

		if(query == 0)
		// insert
			Insert(key, atoi(x));
		else if(query == 1){
		// get
			if( (p = Get(key)) )
				for(sub_p = p->sub; sub_p != NULL; sub_p = sub_p->next)
					printf("%lld\n", sub_p->x);
		}else if(query == 2)
		// delete
			Delete(key);
		else if(query == 3)
		// dump
			Dump(key, x);
	}

	return 0;
}

/*
	<Set_Range_Searchの改良>
	・ハッシュの算出を工夫
	　　各keyにおける最初の4文字が辞書順となるように似非ハッシュ値を計算し、
	　その値を添え字として使う。(説明が難しい)

	・Insert
	該当する添え字の場所へkeyが昇順になるようにデータを挿入することで表示を高速にする

	バケットサイズを「26^5 = 11,881,376」から「26^4 = 456,976」に変更
	テストケースを突破 & 実行時間も短い -> Map_Range_Searchで検証済み

	keyの重複したデータは連結リストへつなぐ
*/
