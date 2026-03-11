#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define rep(i,l,n) for(int i=l;i<n;i++)
#define ll long long
#define ull unsigned long long
#define min(a,b) (a)<(b)?(a):(b)
#define max(a,b) (a)>(b)?(a):(b)

ll P=998244353;
int N;

struct connect{
	int num;
	struct connect *next;
};

struct robot{
	int x;	// robot座標
	int d;	// 座標+稼働距離
	struct connect *next;	// このロボットが稼働した時、必ず稼働するロボット
	int flag;
};

int cmp_robot(const void*,const void*);
void insert(struct robot*,int,int);
ll cal(struct robot*,int);

int main(){
	scanf("%d",&N);
	struct robot *data = (struct robot*)malloc(sizeof(struct robot)*N);
	rep(i,0,N){
		int x,d;
		scanf("%d %d",&x,&d);
		(data+i)->x = x; (data+i)->d = x+d;
		(data+i)->next = NULL;
		(data+i)->flag = 1;
	}
	qsort(data,N,sizeof(struct robot),cmp_robot);
	rep(i,0,N){
		int x = (data+i)->x;
		rep(j,i+1,N){
			if( x < (data+j)->d ){	// このロボットを稼働させるロボットのうち、一番近いもの
				struct connect *temp = (data+j)->next;
				(data+j)->next = (struct connect*)malloc(sizeof(struct connect));
				((data+j)->next)->num = i;
				((data+j)->next)->next = temp;
				break;
			}
		}
	}
	/*
	rep(i,0,N){
		struct robot *temp = data+i;
		rep(j,0,temp->x-(data+N-1)->x) printf(" ");
		printf("@");
		rep(j,0,temp->d-temp->x-1) printf("-");
		printf(">\n");
	}*/
	ll ans = 1;
	for(int i=N-1;i>=0;i--){
		ll cnt = 1;
		if( (data+i)->flag ){
			cnt = cal(data,i);
		}
		ans = ans*cnt%P;
	}
	printf("%lld",ans);
	return 0;
}

// x座標の大きいものを前に
int cmp_robot(const void *a,const void *b){
	struct robot *a0 = (struct robot*)a;
	struct robot *b0 = (struct robot*)b;
	return b0->x - a0->x;
}

// robot a に bを書き込む
void insert(struct robot *data,int a,int b){

	return;
}

ll cal(struct robot *data,int a){
	struct connect *temp = (data+a)->next;
	ll cnt = 1;
	(data+a)->flag = 0;
	while( temp != NULL ){
		cnt = cnt*cal(data,temp->num)%P;
		temp = temp->next;
	}
	return (1+cnt)%P;
}
