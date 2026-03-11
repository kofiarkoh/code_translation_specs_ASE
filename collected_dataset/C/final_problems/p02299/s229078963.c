#include <stdio.h>
#include <stdlib.h>
#include <math.h>
 
#define zahyoutype long
 
typedef struct Point{zahyoutype x,y;}P;
typedef struct line{P p1,p2;}L;
int sgn(zahyoutype x){return x<0?-1:x>0;}
 
//opとoqの内積と外積
zahyoutype innerproduct(P p,P q,P o){return (p.x-o.x)*(q.x-o.x)+(p.y-o.y)*(q.y-o.y);}
zahyoutype crossproduct(P p,P q,P o){return (p.x-o.x)*(q.y-o.y)-(p.y-o.y)*(q.x-o.x);}
 
int iscrossSS(L s,L t){
	//点(p,q)が直線f(x,y)=ax+by+c=0のどっち側にあるかは外積を見ればよい
	zahyoutype t1=crossproduct(t.p1,s.p2,s.p1);
	zahyoutype t2=crossproduct(t.p2,s.p2,s.p1);
	zahyoutype t3=crossproduct(s.p1,t.p2,t.p1);
	zahyoutype t4=crossproduct(s.p2,t.p2,t.p1);
	//同一直線上にない
	if(t1 || t2)return sgn(t1)!=sgn(t2) && sgn(t3)!=sgn(t4);
	//if(t1 || t2)return sgn(t1)*sgn(t2)==-1 && sgn(t3)*sgn(t4)==-1;//端点を含まない
	//同一直線上にある
	//内積をみる
	zahyoutype c1=innerproduct(s.p2,s.p2,s.p1);
	zahyoutype c2=innerproduct(t.p1,s.p2,s.p1);
	zahyoutype c3=innerproduct(t.p2,s.p2,s.p1);
	return 0<=c2&&c2<=c1 || 0<=c3&&c3<=c1 || c2<=0&&c1<=c3 || c3<=0&&c1<=c2;
	//return 0<c2&&c2<c1 || 0<c3&&c3<c1 || c2<=0&&c1<=c3 || c3<=0&&c1<=c2;//端点を含まない
}
 
double seglen(L s){return hypot(s.p2.x-s.p1.x,s.p2.y-s.p1.y);}
double distPP(P p,P q){return hypot(p.x-q.x,p.y-q.y);}
double distPL(P p,L s){return fabs(crossproduct(p,s.p2,s.p1)/seglen(s));}
double distPS(P p,L s){
	zahyoutype c =innerproduct(   p,s.p2,s.p1);
	zahyoutype ss=innerproduct(s.p2,s.p2,s.p1);
	if(c<=0)return distPP(p,s.p1);
	if(c<=ss)return distPL(p,s);
	return distPP(p,s.p2);
}
 
void readpoint(P*p){zahyoutype x=1;scanf(x/2?"%lf%lf":"%ld%ld",&p->x,&p->y);}
void printpoint(P p){zahyoutype x=1;printf(x/2?"%.f %.f\n":"%ld %ld\n",p.x,p.y);}
 
P p[110];
L s[110];
int main(){
	int n;
	scanf("%d",&n);
	for(int i=0;i<n;i++){
		readpoint(p+i);
		s[i].p2=p[i];
		s[(i+1)%n].p1=p[i];
	}
	int q;
	scanf("%d",&q);
	while(q--){
		L t;
		readpoint(&t.p1);
		{
			int flag=0;
			for(int i=0;i<n;i++)if(distPS(t.p1,s[i])==0)flag=1;
			if(flag){
				puts("1");
				continue;
			}
		}
		{
			int flag=1;
			while(flag){
				t.p2.x=rand()%1000+10000;
				t.p2.y=rand()%1000+10000;
				if(seglen(t)==0)continue;
				flag=0;
				for(int i=0;i<n;i++)if(distPS(p[i],t)==0)flag=1;
			}
		}
		 
		int flag2=0;
		for(int i=0;i<n;i++)if(iscrossSS(t,s[i]))flag2^=1;
		puts(flag2?"2":"0");
	}
}
