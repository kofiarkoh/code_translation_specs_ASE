#include <stdio.h>
#include <stdbool.h>

//答えの配列、さいころの目標の目、それの順列セット、p,1,サイコロの目の合計、順列の個数(まぁ720)、サイコロが完成したかどうか
char ans[30005];
int t[7]={0};
int sets[1000][6];
int p,q,tsum,countset,countnum;
bool completed;

//解く、枝刈り、完成したかの判別、順列セット作成
void solve(int nu,int nn,int ns,int ne,int nw,int nin,int now);
bool cut(int inv1,int inv2,int y1,int y2,int z1,int z2);
bool answercheck(int t1,int t2,int t3,int t4,int t5,int t6);
void set();

int main(void){
  int i;
  while(1){
    tsum=0;
    for(i=1;i<=6;i++){
      scanf("%d",&t[i]);
      tsum+=t[i];
    }
    if(tsum==0)break;
    scanf("%d %d",&p,&q);
    countset=0;
    countnum=0;
    set();
    completed=0;
    solve(0,0,0,0,0,0,0);
    //printf("%d\n",countnum);
    if(completed)for(i=p;i<=q;i++)printf("%c",ans[i]);
    else printf("impossible");
    printf("\n");
  }
  return 0;
}

//下、北、南、東、西、インバース（上）、現在の手数
void solve(int nu,int nn,int ns,int ne,int nw,int nin,int now){
 countnum++;
  //printf("%d\n",now);
  //完成したらおわり
  if(completed)return;
  //手数が目標に達したらチェック
  if(now==tsum){
    //printf("%d %d %d %d %d %d\n",nu,nn,ns,ne,nw,nin);
    if(answercheck(nu,nn,ns,ne,nw,nin))completed=1;
    return;
  }
  //これ以上探索しても無駄なら枝刈り
  if(cut(nu,nin,nn,ns,ne,nw)==0){
    // printf("%d %d %d %d %d %d return \n",nu,nn,ns,ne,nw,nin);
    return;
  }
  //ENSWの順番で探索、完成してたらおわり
  ans[now+1]='E';
  solve(ne+1,nn,ns,nin,nu,nw,now+1);
  if(completed)return;
  ans[now+1]='N';
  solve(nn+1,nin,nu,ne,nw,ns,now+1);
  if(completed)return;
  ans[now+1]='S';
  solve(ns+1,nu,nin,ne,nw,nn,now+1);
  if(completed)return;
  ans[now+1]='W';
  solve(nw+1,nn,ns,nu,nin,ne,now+1);
}

//上下ペア、側面２つ
bool cut(int inv1,int inv2,int y1,int y2,int z1,int z2){
  int i,needinv,needy,needz;
  int inv=inv1+inv2,y=y1+y2,z=z1+z2; 
  for(i=0;i<countset;i++){
    needinv = sets[i][0]+sets[i][1]-inv;
    needy = sets[i][2]+sets[i][3]-y;
    needz = sets[i][4]+sets[i][5]-z;
    if(inv1>sets[i][0]||inv2>sets[i][1]||y1>sets[i][2]||y2>sets[i][3]||z1>sets[i][4]||z2>sets[i][5])continue;
    //if(needinv<0||needy<0||needz<0)continue;
    if((needinv<=needy+needz)&&(needy-1<=needinv+needz)&&(needz-1<=needinv+needy))return 1;
  }
  return 0;
}
bool answercheck(int t1,int t2,int t3,int t4,int t5,int t6){
  int i;
  for(i=0;i<countset;i++){
    if(sets[i][0]==t1&&sets[i][1]==t2&&sets[i][2]==t3&&sets[i][3]==t4&&sets[i][4]==t5&&sets[i][5]==t6)return 1;  
  }
  return 0;
}

//6重ループをまわしてつくる。
void set(){
  int a,b,c,d,e,f;
  for(a=1;a<=6;a++){
    for(b=1;b<=6;b++){
      if(a==b)continue;
      for(c=1;c<=6;c++){
	if(a==c||b==c)continue;
	for(d=1;d<=6;d++){
	  if(a==d||b==d||c==d)continue;
	  for(e=1;e<=6;e++){
	    if(a==e||b==e||c==e||d==e)continue;
	    for(f=1;f<=6;f++){
	      if(a==f||b==f||c==f||d==f||e==f)continue;
	      sets[countset][0]=t[a];
	      sets[countset][1]=t[b];
	      sets[countset][2]=t[c];
	      sets[countset][3]=t[d];
	      sets[countset][4]=t[e];
	      sets[countset][5]=t[f];
	      countset++;
	    }
	  }
	}
      }
    }
  }
  //printf("%d\n",countset);
}

