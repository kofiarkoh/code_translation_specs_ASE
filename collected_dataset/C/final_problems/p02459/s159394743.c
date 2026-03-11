#include <stdio.h>
#include <stdlib.h>
#include <string.h>


#define	ll_int	long long int
#define	BUCKET_SIZE	100000


typedef struct data_t{
	char key[21];
	ll_int x;
	struct data_t *next;
} DATA_T;

DATA_T *S[BUCKET_SIZE];


// ハッシュテーブルの初期化
void Init()
{
	int i;

	for(i = 0; i < BUCKET_SIZE; i++)
		S[i] = NULL;
}


ll_int Hash(char *key)
{
	int i;
	int sum = 0;

	for(i = 0; i < strlen(key); i++)
		sum += key[i];

	return sum % BUCKET_SIZE;
}


DATA_T *Get(char *key)
{
	ll_int h;
	DATA_T *p;

	h = Hash(key);

	for(p = S[h]; p != NULL; p = p->next)
		if(!strcmp(key, p->key))
			return p;

	return NULL;
}


void Insert(char *key, ll_int x)
{
	ll_int h;
	DATA_T *p;

	if(( p = Get(key) )){
		p->x = x;
		return;
	}


	if( ( p = (DATA_T *)malloc(sizeof(DATA_T)) ) == NULL){
		printf("malloc error\n");
		exit(1);
	}

	h = Hash(key);
	strcpy(p->key, key);
	p->x = x;
	p->next = S[h];
	S[h] = p;
}


int main()
{
	int q;
	int query;
	char key[21];
	ll_int x;
	DATA_T *p;
	int i;

	scanf("%d", &q);

	for(i = 0; i < q; i++){
		scanf("%d", &query);

		if(query){
		// get
			scanf("%s", key);
			p = Get(key);
			printf("%lld\n", p->x);
		}else{
		// insert
			scanf("%s %lld", key, &x);
			Insert(key, x);
		}
	}

	return 0;
}
