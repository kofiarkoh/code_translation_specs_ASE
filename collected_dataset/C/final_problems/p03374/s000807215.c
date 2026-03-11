//set many funcs template
#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<stdbool.h>
#include<time.h>
#define inf 1072114514
#define llinf 4154118101919364364
#define mod 1000000007
#define pi 3.1415926535897932384

int max(int a,int b){if(a>b){return a;}return b;}
int min(int a,int b){if(a<b){return a;}return b;}
int zt(int a,int b){return max(a,b)-min(a,b);}
int round(int a,int b){if((a%b)*2 >= b){return (a/b)+1;}return a/b;}
int ceil(int a,int b){if(a%b==0){return a/b;}return (a/b)+1;}
int gcd(int a,int b){int c;while(b!=0){c=a%b;a=b;b=c;}return a;}
int lcm(int a,int b){int c=gcd(a,b);a/=c;return a*b;}
int nCr(int a,int b){int i,r=1;for(i=1;i<=b;i++){r*=(a+1-i);r/=i;}return r;}
int fact(int a){int i,r=1;for(i=1;i<=a;i++){r*=i;}return r;}
int pow(int a,int b){int i,r=1;for(i=1;i<=b;i++){r*=a;}return r;}
long long llmax(long long a,long long b){if(a>b){return a;}return b;}
long long llmin(long long a,long long b){if(a<b){return a;}return b;}
long long llzt(long long a,long long b){return llmax(a,b)-llmin(a,b);}
long long llround(long long a,long long b){if((a%b)*2 >= b){return (a/b)+1;}return a/b;}
long long llceil(long long a,long long b){if(a%b==0){return a/b;}return (a/b)+1;}
long long llgcd(long long a,long long b){long long c;while(b!=0){c=a%b;a=b;b=c;}return a;}
long long lllcm(long long a,long long b){long long c=llgcd(a,b);a/=c;return a*b;}
long long llnCr(long long a,long long b){long long i,r=1;for(i=1;i<=b;i++){r*=(a+1-i);r/=i;}return r;}
long long llfact(long long a){long long i,r=1;for(i=1;i<=a;i++){r*=i;}return r;}
long long llpow(long long a,long long b){long long i,r=1;for(i=1;i<=b;i++){r*=a;}return r;}
double dbmax(double a,double b){if(a>b){return a;}return b;}
double dbmin(double a,double b){if(a<b){return a;}return b;}
double dbzt(double a,double b){return dbmax(a,b)-dbmin(a,b);}
int sortfncsj(const void *a,const void *b){if(*(int *)a>*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int sortfnckj(const void *a,const void *b){if(*(int *)a<*(int *)b){return 1;}if(*(int *)a==*(int *)b){return 0;}return -1;}
int llsortfncsj(const void *a,const void *b){if(*(long long *)a>*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int llsortfnckj(const void *a,const void *b){if(*(long long *)a<*(long long *)b){return 1;}if(*(long long *)a==*(long long *)b){return 0;}return -1;}
int dbsortfncsj(const void *a,const void *b){if(*(double *)a>*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int dbsortfnckj(const void *a,const void *b){if(*(double *)a<*(double *)b){return 1;}if(*(double *)a==*(double *)b){return 0;}return -1;}
int strsortfncsj(const void *a,const void *b){return strcmp((char *)a,(char *)b);}
int strsortfnckj(const void *a,const void *b){return strcmp((char *)b,(char *)a);}

typedef struct{
long long single;
long long score;
}smem;

int sortfnc(const void *a,const void *b){
if(((smem*)a)->single == ((smem*)b)->single){return 0;}
if(((smem*)a)->single < ((smem*)b)->single){return -1;}
return 1;
}

int main(void){
    smem sushi[262144];
    long long i,j,n,c,nr,r=0,np;
    long long rw,zsw=0,ksw=0,zdw=0,kdw=0;
    double d;
    char s[262144];
    scanf("%lld%lld",&n,&c);
    //l=strlen(s);
    for(i=0;i<n;i++){scanf("%lld%lld",&sushi[i].single,&sushi[i].score);}
    qsort(sushi,n,sizeof(smem),sortfnc);
    nr=0;np=0;rw=0;
    for(i=0;i<n;i++){
        nr-=llzt(np,sushi[i].single);np=sushi[i].single;
        nr+=sushi[i].score;
        r=llmax(r,nr);
        rw+=sushi[i].score;
        if(2*sushi[i].single<=c){
            zsw=llmax(zsw,rw-sushi[i].single);
            zdw=llmax(zdw,rw-(sushi[i].single*2));
        }
        sushi[i].single=c-sushi[i].single;
    }
    qsort(sushi,n,sizeof(smem),sortfnc);
    nr=0;np=0;rw=0;
    for(i=0;i<n;i++){
        nr-=llzt(np,sushi[i].single);np=sushi[i].single;
        nr+=sushi[i].score;
        r=llmax(r,nr);
        rw+=sushi[i].score;
        if(2*sushi[i].single<c){
            ksw=llmax(ksw,rw-sushi[i].single);
            kdw=llmax(kdw,rw-(sushi[i].single*2));
        }
        sushi[i].single=c-sushi[i].single;
    }
    //qsort(sushi,n,sizeof(smem),sortfnc);
    printf("%lld\n",llmax(r,llmax(zsw+kdw,zdw+ksw)));
    return 0;
}