#include <stdio.h>
#include <stdlib.h>


#define	ll_int	long long int
#define	BUCKET_SIZE	100000


int ELEMENT_N;


typedef struct data_t{
	ll_int num;
	int n;
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


ll_int Hash(ll_int num)
{
	return num % BUCKET_SIZE;
}


DATA_T *Find(ll_int num)
{
	ll_int h;
	DATA_T *p;


	h = Hash(num);

	for(p = S[h]; p != NULL; p = p->next)
		if(num == p->num)
			return p;

	return NULL;
}


void Insert(ll_int num)
{
	ll_int h;
	DATA_T *p;


	if(( p = Find(num) )){
		p->n++;
		return;
	}


	if( ( p = (DATA_T *)malloc(sizeof(DATA_T)) ) == NULL){
		printf("malloc error\n");
		exit(1);
	}

	h = Hash(num);
	p->num = num;
	p->n = 1;
	p->next = S[h];
	S[h] = p;
	ELEMENT_N++;
}


int main()
{
	int q;
	int query;
	ll_int num;
	DATA_T *p;
	int i;


	ELEMENT_N = 0;

	scanf("%d", &q);

	for(i = 0; i < q; i++){
		scanf("%d", &query);

		if(query){
		// find
			scanf("%lld", &num);
			if(( p = Find(num) ))
				printf("1\n");
			else
				printf("0\n");
		}else{
		// insert
			scanf("%lld", &num);
			Insert(num);
			printf("%d\n", ELEMENT_N);
		}
	}

	return 0;
}
