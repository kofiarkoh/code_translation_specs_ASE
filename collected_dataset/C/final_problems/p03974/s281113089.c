#include<stdio.h>
#include<stdlib.h>

#define F 26

typedef struct trieNode{
  int cnt;
  struct trieNode *next[F];
} trie;

trie* newNode(void){
  static trie *buf=NULL;
  if(buf==NULL) buf=(trie *)malloc(sizeof(trie)*(400000+100000+1));
  trie *res=buf++;
  res->cnt=0;
  int i;
  for(i=0;i<F;i++) res->next[i]=NULL;
  return res;
}

void add(trie *r,char *s){
  while(1){
    r->cnt++;
    if(*s=='\0') break;
    if(r->next[*s-'a']==NULL) r->next[*s-'a']=newNode();
    r=r->next[*s-'a'];
    s++;
  }
  return;
}

int count(trie *r){
  return r==NULL?0:r->cnt;
}

void calc(trie *r,char *s,int *cnt,int *before){
  while(*s!='\0'){
    int local=r->cnt;
    int i;
    for(i=0;i<F;i++){
      int c=count(r->next[i]);
      local-=c;
      if(*s!='a'+i) cnt[(*s-'a')*F+i]+=c;
    }
    *before+=local;
    r=r->next[*s-'a'];
    s++;
  }
  return;
}

void run(void){
  int n;
  scanf("%d",&n);
  char **sp=(char **)malloc(sizeof(char *)*n);
  char *s=(char *)malloc(sizeof(char)*(400000+n));
  trie *root=newNode();
  char *p=s;
  int i;
  for(i=0;i<n;i++){
    scanf("%s",p);
    add(root,p);
    sp[i]=p;
    while(*p!='\0') p++;
    p++;
  }
  int *before=(int *)calloc(n,sizeof(int));
  int *cnt=(int *)calloc(F*F*n,sizeof(int));
  for(i=0;i<n;i++) calc(root,sp[i],cnt+F*F*i,before+i);
  int q;
  scanf("%d",&q);
  while(q--){
    int k;
    char pe[27];
    scanf("%d%s",&k,pe);
    k--;
    int ans=before[k];
    int j;
    for(i=F-1;i>=0;i--){
      for(j=i-1;j>=0;j--){
	ans+=cnt[(k*F+pe[i]-'a')*F+pe[j]-'a'];
      }
    }
    printf("%d\n",ans+1);
  }
}

int main(void){
  run();
  return 0;
}
