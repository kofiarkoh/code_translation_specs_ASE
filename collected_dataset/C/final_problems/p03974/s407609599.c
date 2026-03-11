#include<stdio.h>
#include<string.h>

#define lk 32

typedef struct{
  int lp;
  int par;
  int bkw;
  int chi[lk];
}trn;

int trc=0;
trn trie[524288];

void trinit(){
  int i;
  trie[0].lp=0;
  trie[0].par=-1;
  trie[0].bkw=-1;
  for(i=0;i<lk;i++){
    trie[0].chi[i]=-1;
  }
  trc=1;
}

void crn(int x,int pt){
  int i;
  trie[x].chi[pt]=trc;
  trie[trc].par=x;
  trie[trc].bkw=pt;
  trie[trc].lp=trie[x].lp+1;
  for(i=0;i<lk;i++){
    trie[trc].chi[i]=-1;
  }
  trc++;
}

int run(char s[]){
  int i,l=strlen(s),v=0,c;
  for(i=0;i<=l;i++){
    if(i!=l){c=s[i]-'`';}
    else{c=0;}
    if(trie[v].chi[c]==-1){
      crn(v,c);
    }
    v=trie[v].chi[c];
  }
  return v;
}

void tricomp(){
  int i,j,c,tac;
  for(i=0;i<trc;i++){
    if(trie[i].par==-1){continue;}
    c=0;
    for(j=0;j<lk;j++){
      if(trie[i].chi[j]!=-1){tac=trie[i].chi[j];c++;}
    }
    if(c==1){
      trie[trie[i].par].chi[trie[i].bkw]=tac;
      trie[tac].par=trie[i].par;
      trie[tac].bkw=trie[i].bkw;
    }
  }
}

int main(){
  trinit();
  int n,i,j,w[524288]={0},id[524288]={0},q,k,res,v,vp,f;
  char s[524288],p[32];
  scanf("%d",&n);
  for(i=1;i<=n;i++){
    scanf("%s",s);
    id[i]=run(s);
    w[id[i]]++;
  }
  tricomp();
  for(i=trc-1;i>=0;i--){
    if(trie[i].par==-1){continue;}
    w[trie[i].par]+=w[i];
  }
  scanf("%d",&q);
  for(i=0;i<q;i++){
    scanf("%d%s",&k,p);
    res=0;
    v=id[k];
    while(trie[v].par!=-1){
      vp=trie[v].par;
      f=0;
      if(trie[v].bkw==0){f=1;}
      for(j=0;j<26;j++){
        if(f==1 && trie[vp].chi[p[j]-'`']!=-1){
          res+=w[trie[vp].chi[p[j]-'`']];
        }
        if(trie[v].bkw==p[j]-'`'){f=1;}
      }
      v=vp;
    }
    printf("%d\n",n-res);
  }
}