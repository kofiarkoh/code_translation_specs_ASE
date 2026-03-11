//not conflict with math.h template
//int -> leading i
//max -> imax
//
//llround -> ll_round

#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<stdbool.h>
#include<math.h>
#define inf 1072114514
#define llinf 4154118101919364364
#define mod 1000000007
#define pi 3.1415926535897932384

int imax(int a,int b){if(a>b){return a;}return b;}
int imin(int a,int b){if(a<b){return a;}return b;}
int izt(int a,int b){return imax(a,b)-imin(a,b);}
int iround(int a,int b){if((a%b)*2 >= b){return (a/b)+1;}return a/b;}
int iceil(int a,int b){if(a%b==0){return a/b;}return (a/b)+1;}
int igcd(int a,int b){int c;while(b!=0){c=a%b;a=b;b=c;}return a;}
int ilcm(int a,int b){int c=igcd(a,b);a/=c;return a*b;}
int inCr(int a,int b){int i,r=1;for(i=1;i<=b;i++){r*=(a+1-i);r/=i;}return r;}
int ifact(int a){int i,r=1;for(i=1;i<=a;i++){r*=i;}return r;}
int ipow(int a,int b){int i,r=1;for(i=1;i<=b;i++){r*=a;}return r;}
long long llmax(long long a,long long b){if(a>b){return a;}return b;}
long long llmin(long long a,long long b){if(a<b){return a;}return b;}
long long llzt(long long a,long long b){return llmax(a,b)-llmin(a,b);}
long long ll_round(long long a,long long b){if((a%b)*2 >= b){return (a/b)+1;}return a/b;}
long long llceil(long long a,long long b){if(a%b==0){return a/b;}return (a/b)+1;}
long long llgcd(long long a,long long b){long long c;while(b!=0){c=a%b;a=b;b=c;}return a;}
long long lllcm(long long a,long long b){long long c=llgcd(a,b);a/=c;return a*b;}
long long llnCr(long long a,long long b){long long i,r=1;for(i=1;i<=b;i++){r*=(a+1-i);r/=i;}return r;}
long long llfact(long long a){long long i,r=1;for(i=1;i<=a;i++){r*=i;}return r;}
long long llpow(long long a,long long b){long long i,r=1;for(i=1;i<=b;i++){r*=a;}return r;}
double dbmax(double a,double b){if(a>b){return a;}return b;}
double dbmin(double a,double b){if(a<b){return a;}return b;}
double dbzt(double a,double b){return dbmax(a,b)-dbmin(a,b);}
int isortfncsj(const void *a,const void *b){if(*(int *)a>*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int isortfnckj(const void *a,const void *b){if(*(int *)a<*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int llsortfncsj(const void *a,const void *b){if(*(long long *)a>*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int llsortfnckj(const void *a,const void *b){if(*(long long *)a<*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int dbsortfncsj(const void *a,const void *b){if(*(double *)a>*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int dbsortfnckj(const void *a,const void *b){if(*(double *)a<*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int strsortfncsj(const void *a,const void *b){return strcmp((char *)a,(char *)b);}
int strsortfnckj(const void *a,const void *b){return strcmp((char *)b,(char *)a);}

int thn(int x1,int y1,int x2,int y2){
    int xs,ys;
    xs=izt(x1,x2);
    ys=izt(y1,y2);
    return xs*xs+ys*ys;
}

double sm(int x1,int y1,int x2,int y2,int x3,int y3){
    double r;
    x2-=x1;x3-=x1;
    y2-=y1;y3-=y1;
    r=izt(x2*y3,x3*y2);
    r/=2.0f;
    return r;
}

int main(void){
    int i,j,x[4],y[4];
    double d[4],ss,kh,rk;
    char s[262144];
    scanf("%d%d",&x[0],&y[0]);
    scanf("%d%d",&x[1],&y[1]);
    scanf("%d%d",&x[2],&y[2]);
    d[0]=sqrt(thn(x[0],y[0],x[1],y[1]));
    d[1]=sqrt(thn(x[0],y[0],x[2],y[2]));
    d[2]=sqrt(thn(x[1],y[1],x[2],y[2]));
    ss=sm(x[0],y[0],x[1],y[1],x[2],y[2]);
    qsort(d,3,sizeof(double),dbsortfnckj);
    kh=(ss*2.0f)/d[0];
    rk=kh+((d[0]+d[1]+d[2])/2.0f);
    ss/=rk;
    printf("%.12lf\n",ss);
    return 0;
}