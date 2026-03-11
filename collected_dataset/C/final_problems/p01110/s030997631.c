#include <stdio.h>

#define MAX 500 //配列の大きさは500*500で十分です。

int main(void){
	while(1){
		//問題ではn, m, t, pとして与えられますが、
		//nはw、mはhとして変数を用意します。
		int w, h, t, p; 
		scanf(" %d %d %d %d", &w, &h, &t, &p);
		if(!w && !h && !t && !p) break;

		//2次元配列は行列のように、i行j列(kami[i][j])として考えます。
		//そのため、座標(x, y)は kami[y][x] となります。
		int kami[MAX][MAX] = {0};
		for(int i=0; i<h; i++){
			for(int j=0; j<w; j++){
				kami[i][j] = 1;//折り紙が存在する部分を1で初期化
			}
		}

		int origin_x = 0, origin_y =0; //折り紙左上の角の座標を原点として記憶(kami[origin_y][origin_x])
		for(int k=0; k<t; k++){
			int d, c;
			scanf(" %d %d", &d, &c);

			if(d == 1){
				//もし d が 1 なら，左端から c だけ右側を通る垂直の折り線の左側を右側に折り重ねる．
				int dist = 1; //distは折られる前の座標と、折る先の座標の差の絶対値。
				for(int j=origin_x+c; j<origin_x+(2*c); j++){	//折られる先のx座標
					for(int i=origin_y; i<MAX; i++){	//垂直方向に折るループ
						kami[i][j] += kami[i][j-dist];
					}
					dist += 2;
				}
				origin_x += c;
			} else{
				//このプログラムの場合は、上下を入れ替えています。
				//	→もし d が 2 なら，上端から c だけ下側を通る水平の折り線の上側を下側に折り重ねる。
				int dist = 1; //distは折られる前の座標と、折る先の座標の差の絶対値。
				for(int i=origin_y+c; i<origin_y+(2*c); i++){	//折られる先のy座標
					for(int j=origin_x; j<MAX; j++){	//水平方向に折るループ
						kami[i][j] += kami[i-dist][j];
					}
					dist += 2;
				}
				origin_y += c;
			}
		}

		for(int k=0; k<p; k++){
			int x, y;
			scanf(" %d %d", &x, &y);
			printf("%d\n", kami[origin_y+y][origin_x+x]);
		}
	}

	return 0;
}
