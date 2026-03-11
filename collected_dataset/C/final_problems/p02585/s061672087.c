#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define rep(i,N) for(int i=0;i<(int)N;i++)
const long LMIN=-1l<<63;
struct TAG{long* array;int size,head,tail,count;};
typedef struct TAG* DEQUE;
static inline void PUT(char c)
{
  static char buf[1<<15],*ptr=buf;
  if(ptr==buf+strlen(buf)||c==0){fwrite(buf,1,ptr-buf,stdout),ptr=buf;}*ptr++=c;
}
static inline long IN(void)
{
  long x=0,f=0,c=getchar();while(c<48||c>57){f^=c==45,c=getchar();}
  while(c>47&&c<58){x=x*10+c-48,c=getchar();}return f?-x:x;
}
static inline void OUT(long a)
{
  if(a<0)PUT('-'),a=-a;
  int d[40],i=0;do{d[i++]=a%10;}while(a/=10);
  while(i--){PUT(d[i]+48);}PUT('\n');
}
static inline long Max(const long x,const long y){return x<y?y:x;}
static inline void* xmalloc(size_t size){void* p=malloc(size);if(p==NULL){exit(EXIT_FAILURE);}return p;}
static inline DEQUE CREATE(void)
{
  const int size=10000;
  struct TAG* deque=(DEQUE)xmalloc(sizeof(struct TAG));
  deque->array=(long*)xmalloc(sizeof(long)*size);
  deque->size=size;deque->head=deque->tail=deque->count=0;return deque;
}
static inline void DELETE(DEQUE deque){free(deque->array);free(deque);}
static inline int PREV(DEQUE deque,int index){return(!index)?deque->size-1:index-1;}
static inline int NEXT(DEQUE deque,int index){return(index+1)%deque->size;}
static inline void PUSH(DEQUE deque,long val,int back)
{
  //front=0,back=1;
  if(deque->count==deque->size){exit(EXIT_FAILURE);}
  if(back)deque->tail=NEXT(deque,deque->tail);else deque->head=PREV(deque,deque->head);
  deque->array[(back)?deque->tail:deque->head]=val;deque->count++;
  if(deque->count==1){if(back)deque->head=deque->tail;else deque->tail=deque->head;}
}
static inline long POP(DEQUE deque,int back)
{
  if(!deque->count){exit(EXIT_FAILURE);}
  long val=deque->array[(back)?deque->tail:deque->head];
  if(back)deque->tail=PREV(deque,deque->tail);else deque->head=NEXT(deque,deque->head);
  deque->count--;return val;
}
int main(void)
{
  int N=IN(),K=IN(),C[5000],P[5000],vis[5000]={0};long score=LMIN,vector[15000];
  rep(i,N){P[i]=IN()-1;}rep(i,N){C[i]=IN();score=Max(score,C[i]);}if(score<0){return OUT(score),0;}
  rep(i,N)
  {
    if(vis[i]){continue;}
    int len=0,now=K,sz=0,x=P[i];long high=LMIN,temp=0,y=1l*C[x];vis[i]=1;
    vector[sz++]=0,vector[sz++]=y;
    while(!vis[x]){vis[x]=1;x=P[x];y+=1l*C[x];vector[sz++]=1l*C[x];}
    if(y>0)
    {
      temp+=1l*Max(1l*now/(sz-1l)-1l,0l)*y;
      if(now>sz-1){now%=sz-1;now+=1l*sz-1l;}
    }
    len=sz;
    rep(j,len-1){vector[sz++]=1l*vector[j+1];}
    rep(j,len-1){vector[sz++]=1l*vector[j+1];}
    rep(j,sz-2){vector[j+2]+=1l*vector[j+1];}
    DEQUE dq=CREATE();PUSH(dq,0,0);
    rep(j,sz-1)
    {
      while(dq->count&&vector[dq->array[dq->head]]>vector[j+1]){POP(dq,0);}
      PUSH(dq,j+1,0);
      while(dq->count&&j+1-dq->array[dq->tail]>now){POP(dq,1);}
      if((j+1)!=dq->array[dq->tail]){high=1l*Max(1l*high,1l*vector[j+1]-1l*vector[dq->array[dq->tail]]);}
    }
    temp+=1l*high;score=1l*Max(score,temp);DELETE(dq);
  }
  return OUT(score),0;
}