#include <stdio.h>
#include <limits.h>
#include <stdlib.h>
#include <math.h>


#define MAX_N 4100000
#define MIN(a,b) (a>b?b:a)

typedef long long ll;
// ZOgØðÂO[ozñ
// 0 <-> h*w-1ÌindexÉÀf[^ª¶Ý
ll dat[2*MAX_N-1];
int h,w;
const ll INF=1LL<<40;
// ú»
void init(int h_,int w_){
    h=1;
    w=1;
    while(h<h_)
        h*=2;
    while(w<w_)
        w*=2;
    int i;
    for(i = 0; i<2*h*w-1; i++)
      dat[i]=INF;
    //fill(dat,dat+2*h*w-1,INF);
}
// kÔÚÌl(0-indexed)ðaÉÏX
void update(int k,int a){
    // tÌß_
    k+=(h*w-1);
    dat[k]=a;
    // ÌÚèÈªçXV
    while(k>0){
        k=(k-1)/2;
        dat[k]=MIN(dat[k*2+1],dat[k*2+2]);
    }
}
ll query(int qminx,int qminy,int qmaxx,int qmaxy,int k,int minx,int miny,int maxx,int maxy){
    // dÈçÈ¢
    if(qmaxx<minx||maxx<qminx||qmaxy<miny||maxy<qminy)
        return INF;
    // ¡ñÌÌæª®SÉÜÜêé
    else if(minx>=qminx&&qmaxx>=maxx&&miny>=qminy&&qmaxy>=maxy)
        return dat[k];
    ll vl,vr;
    // ÅÍ¡Éª
    if(miny!=maxy){
        int midY=(miny+maxy)/2;
        vl=query(qminx,qminy,qmaxx,qmaxy,k*2+1,minx,miny,maxx,midY);
        vr=query(qminx,qminy,qmaxx,qmaxy,k*2+2,minx,midY+1,maxx,maxy);
    }
    // ªµ«Á½çc
    else{
        int midX=(minx+maxx)/2;
        vl=query(qminx,qminy,qmaxx,qmaxy,k*2+1,minx,miny,midX,maxy);
        vr=query(qminx,qminy,qmaxx,qmaxy,k*2+2,midX+1,miny,maxx,maxy);
    }
    return MIN(vl,vr);
}
int r,cc,q,t;

int main(){
    while(scanf("%d %d %d\n",&r,&cc,&q)&&!(r==0&&cc==0&&q==0)){
        int a,b,c,d;
        init(r,cc);
	int i,j;
        for(i = 0; i < r; i++){
            for(j = 0; j < cc; j++){
                scanf("%d",&t);
                update(i*w+j,t);
            }
        }
        for(i = 0; i < q; i++){
            scanf("%d %d %d %d\n",&a,&b,&c,&d);
            printf("%lld\n",query(b,a,d,c,0,0,0,w-1,h-1));
        }
    }
    return 0;
}