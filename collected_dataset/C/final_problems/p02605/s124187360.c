#include <stdio.h>
#include <stdlib.h>
#define rep(i,l,r)for(int i=(l);i<(r);i++)
#define min(p,q)((p)<(q)?(p):(q))

typedef struct tuple{int x,y,d;}T;
 
int xcmp(const void*p,const void*q){
	T *pp=(T*)p,*qq=(T*)q;
	if(pp->x<qq->x)return-1;
	if(pp->x>qq->x)return 1;
	if(pp->y<qq->y)return-1;
	if(pp->y>qq->y)return 1;
	return 0;
}

int ans=1e9;
T temp[400010];
void solve(int c){
	qsort(temp,c,sizeof(T),xcmp);
	rep(i,1,c)if(temp[i-1].x==temp[i].x&&temp[i-1].d==1&&temp[i].d==-1)ans=min(ans,abs(temp[i-1].y-temp[i].y)*5);
}

T dataaaa[4][200010];
T *data[4];
int main(){
	rep(i,0,4)data[i]=dataaaa[i];
	int n;
	scanf("%d",&n);
	int cnt[4]={};
	rep(i,0,n){
		int x,y;
		char c;
		scanf("%d%d %c",&x,&y,&c);
		T*t;
		if(c=='U')t=data[0]+cnt[0]++;
		if(c=='L')t=data[1]+cnt[1]++;
		if(c=='D')t=data[2]+cnt[2]++;
		if(c=='R')t=data[3]+cnt[3]++;
		t->x=x;
		t->y=y;
	}
	
	rep(d,0,4){
		//正面衝突
		int c=0;
		rep(i,0,cnt[0]){
			temp[c].x=data[0][i].x;
			temp[c].y=data[0][i].y;
			temp[c].d=1;
			c++;
		}
		rep(i,0,cnt[2]){
			temp[c].x=data[2][i].x;
			temp[c].y=data[2][i].y;
			temp[c].d=-1;
			c++;
		}
		solve(c);
		
		//UとLの衝突
		c=0;
		rep(i,0,cnt[0]){//U
			temp[c].x=data[0][i].x-data[0][i].y;
			temp[c].y=data[0][i].x+data[0][i].y;
			temp[c].d=1;
			c++;
		}
		rep(i,0,cnt[1]){//L
			temp[c].x=data[1][i].x-data[1][i].y;
			temp[c].y=data[1][i].x+data[1][i].y;
			temp[c].d=-1;
			c++;
		}
		solve(c);
		
		//rotate
		rep(d,0,4)rep(i,0,cnt[d]){
			T old=data[d][i];
			data[d][i].x=-old.y;
			data[d][i].y= old.x;
		}
		{T* aaa=data[0];data[0]=data[3];data[3]=data[2];data[2]=data[1];data[1]=aaa;}
		{int aaa=cnt[0];cnt[0]=cnt[3];cnt[3]=cnt[2];cnt[2]=cnt[1];cnt[1]=aaa;} 
	}
	
	if(ans==1e9)puts("SAFE");
	else printf("%d",ans);
}