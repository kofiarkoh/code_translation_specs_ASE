/* aoj1350  */
//提出用(アクセプトされました)
#include<stdio.h>
#include <stdlib.h>  // qsort
#define N 500  //島の数の最大
#define M 50000 //橋の数の最大

int mst[M]={0};  //最初に作った最小木の橋を記憶する
int flag=0;      //最小木が1度作られたかどうか
int rm = -1;   //取り除いた橋の番号(初期値-1)

//橋を定義
typedef struct bridge{  
    //始点の島,終点の島,コスト
    int s; int d; int c;
}bridge_t;

//比較関数(qsort用)
int cmp(const void *a,const void *b){
  return ((bridge_t*)a)->c - ((bridge_t*)b)->c;
}

//与えられた要素の木の根を再帰で求める関数
int u_root(int x,int number[]){
    //根
    if(number[x-1] == x)
        return x;

    //経路圧縮
    else
        return number[x-1] = u_root(number[x-1],number);
}

//xとyが同じ集合に属するか否かを根を見て判断する(同じなら1を返す)
int u_find(int x, int y, int number[]){
    return u_root(x,number) == u_root(y,number);
}

//xとyの属する集合を併合
void u_union(int x, int y, int number[]){     //x,yは島の番号(1~)を表す
    number[y-1] = x;
}

//全ての要素が同じ集合に含まれているかどうかをfindで判定する関数
int allsame(int n, int number[])
{
    int y;
    for(y=2;y<=n;y++)
            if(!u_find(1,y,number)) //一つでも同じ集合に含まれない要素があるなら0を返す
                return 0;
    return 1; //全ての要素が同じ集合に含まれていれば1を返す。
}


//クラスカル法で最小全域木を生成する関数
//異常終了(最小木が生成できなかった)の時は0を返す
int kruskal(bridge_t b[], int n, int m, int number[]){
    int i=0; //配列の添字
    int mc=0;  //最小木のコスト
    //独立した点がなくなるまで繰り返す
    while(!allsame(n,number)){
        //独立していてかつ取り除かれた橋でないならその要素の根を合成する
        if(!u_find(b[i].s,b[i].d,number) && i!=rm){
            u_union(u_root(b[i].s,number),u_root(b[i].d,number),number);
            //まだ一度も最小木が作られていないならどの橋を使ったかを記憶する
            if(!flag)
                mst[i] = 1;
            //コストを足していく
            mc += b[i].c;
        }
        i++;
        //最後まで探索して、一つでも同じ集合にない要素があれば、最小木は作れないということになる
        if(i==m)
          if(!allsame(n,number))
             return 0;
    }
    return mc;
}

int main(void){
    int n,m;        //島の数,橋の数
    bridge_t b[M];  //橋の情報
    int mc;         //最初に作った最小木のコスト
    int mc2;        //2回目以降に作る最小木のコスト
    bridge_t temp;  //橋の情報の一時記憶領域
    int number[N];  //親の番号
    int alt[M] = {0};     //オルタナティブな橋の番号(初期値は全て非オルタナティブ))
    int acost=0,abridge=0;    //オルタナティブな橋のコストの和と、オルタナティブな橋の本数の和
    int i,j;

    scanf("%d%d",&n,&m);
    //n要素で初期化(初めは全ての頂点が根)
    for(i=0;i<n;i++)
        number[i] = i+1;
    //橋の始点、終点、コストを入力
    for(i=0;i<m;i++){
        scanf("%d%d%d",&b[i].s,&b[i].d,&b[i].c);
    }
    //qsortによるソート(コストの昇順)
    qsort(b,m,sizeof(bridge_t),cmp);

    //クラスカル法を行う関数を呼び出し
    //全ての要素で最小木を作る
    mc = kruskal(b,n,m,number);
    flag=1;     //１度最小木を作成した

    //numberを初期化
    for(i=0;i<n;i++){
        number[i] = i+1;
    }
    //作った最小木の中で辺を一つだけ取り除き最小木が作れるかどうかで場合分け
    for(j=0;j<m;j++){
        //最初に作った最小木の橋ならばその橋を取り除く
        if(mst[j]){
            rm = j;   //取り除いた橋の番号を記憶
            mc2 = kruskal(b,n,m,number);    //橋を一つ取り除いて最小木を作る
            //ある橋を一つ取り除いて同じコストの最小木を作れないなら、それは不変の橋である
            if(mc < mc2 || mc2 == 0)
                alt[j]=1;
            //numberを初期化しておく
            for(i=0;i<n;i++)
                number[i] = i+1;
        }
        //オルタナティブな橋が見つかれば
        if(alt[j]){
            abridge++;
            acost += b[j].c;
        }
    }
    printf("%d %d\n",abridge,acost);
    
    return 0;
}
