#include<stdio.h>
#define inf 1072114514
#define size 262144

void prarr(int arr[],int n){
  int i;
  for(i=0;i<n;i++){
    if(i){printf(" ");}
    printf("%d",arr[i]);
  }
  printf("\n");
  return;
}

typedef struct{
int val;
int node;
}data;

int ssize=size;
data vd;

void sinit(data stree[]){
    vd.val=inf;
    vd.node=-1;
    int i,j;
    for(i=0;i<2*ssize;i++){
        stree[i].val=inf;
        stree[i].node=-1;
    }
}

void upd(data stree[],int k,data a){
    int i,m;
    data ld,rd,md;
    k+=ssize-1;
    stree[k]=a;
    while(k>0){
        k=(k-1)/2;
        ld=stree[2*k+1];
        rd=stree[2*k+2];
        if(ld.val<rd.val){md=ld;}
        else{md=rd;}
        stree[k]=md;
    }
}

//calc min[a,b)
//call:k=0,l=0,r=ssize
data query(data stree[],int a,int b,int k,int l,int r){
    data ld,rd;
    if(r<=a || b<=l){return vd;}
    if(a<=l && r<=b){
        return stree[k];
    }
    else{
        ld=query(stree,a,b,k*2+1,l,(l+r)/2);
        rd=query(stree,a,b,k*2+2,(l+r)/2,r);
        if(ld.val<rd.val){return ld;}
        else{return rd;}
    }
}

int n;
int a[size];
data seg[2][2*size];
int res[size];
int mr[size];

void solve(int st,int fi,int pt){
  if(fi-st==2){
    res[pt]=a[st];
    res[pt+1]=a[st+1];
    return;
  }
  int i,w,p;
  int fp,sp,tp,lp;
  data fm,sm;
  w=st%2;
  fm=query(seg[w],st,fi+1,0,0,ssize);
  p=fm.node;
  w=1-w;
  sm=query(seg[w],p+1,fi+1,0,0,ssize);
  res[pt]=fm.val;
  res[pt+1]=sm.val;
  pt+=2;
  fp=pt;
  if(st<fm.node-1){solve(st,fm.node-1,fp);}
  pt+=((fm.node-st));
  sp=pt;
  if(fm.node+1<sm.node-1){solve(fm.node+1,sm.node-1,sp);}
  pt+=((sm.node-(fm.node+1)));
  tp=pt;
  if(sm.node+1<fi){solve(sm.node+1,fi,tp);}
  pt+=(fi-sm.node);
  lp=pt;

  int nf=fp,ns=sp,nt=tp,c=0,v;
  while(nf<sp || ns<tp || nt<lp){
    v=inf;
    if(nf<sp){
      if(res[nf]<v){
        v=res[nf];
        w=1;
      }
    }

    if(ns<tp){
      if(res[ns]<v){
        v=res[ns];
        w=2;
      }
    }

    if(nt<lp){
      if(res[nt]<v){
        v=res[nt];
        w=3;
      }
    }

    if(w==1){
      mr[c]=res[nf];
      c++;
      nf++;
      mr[c]=res[nf];
      c++;
      nf++;
    }
    if(w==2){
      mr[c]=res[ns];
      c++;
      ns++;
      mr[c]=res[ns];
      c++;
      ns++;
    }
    if(w==3){
      mr[c]=res[nt];
      c++;
      nt++;
      mr[c]=res[nt];
      c++;
      nt++;
    }
  }
  for(i=0;i<c;i++){
    res[fp]=mr[i];
    fp++;
  }
  return;
}

int main(void){
    int i,j,m,k,b,c,h,w,r=0,l,t;
    data pd;
    sinit(seg[0]);
    sinit(seg[1]);
    scanf("%d",&n);
    for(i=0;i<n;i++){
      scanf("%d",&a[i]);
      if(i%2==0){
        pd.val=a[i];
        pd.node=i;
        upd(seg[0],i,pd);
        pd.val=inf;
        upd(seg[1],i,pd);
      }
      else{
        pd.val=a[i];
        pd.node=i;
        upd(seg[1],i,pd);
        pd.val=inf;
        upd(seg[0],i,pd);
      }
    }
    //printf("ok\n");
    solve(0,n-1,0);
    prarr(res,n);
    return 0;
}
