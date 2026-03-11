
#include "stdio.h"

int main()
{
	//変数を立てて初期化
	//それぞれ、見た回数、比の高橋側と青木側、票数の高橋側と青木側
	int N = 0;
	int T[1010], A[1010];
	long long int voteT = 0, voteA = 0;
	for (int i = 0; i < 1010; i++) {
		T[i] = 0;
		A[i] = 0;
	}

	//見た回数と比を入力
	scanf("%d",&N);
	for (int i = 0; i < N; i++) {
		scanf("%d %d", &T[i], &A[i]);
	}

	//投票数の初期値は１回目に見たときの票数
	voteT = T[0];
	voteA = A[0];

	//ループ回数確認用の変数
	int count = 0;

	//１回目に見たときの投票数を初期値としてるのでi=1でi=Nまでループ
	for (int i = 1; i < N; i++) {
		//カウンタを初期化する
		count = 0;

		//小細工そのいち
		//比が前と一致するなら投票数が変化していないのでwhileをスキップする
		if ((T[i] == T[i - 1]) && (A[i] == A[i - 1]))
			count = -1;

		//小細工そのに
		//１：１なら大きいほうに合わせる
		if ((T[i] == 1) && (A[i] == 1)) {
			if (voteT >= voteA)
				voteA = voteT;
			else
				voteT = voteA;

			count = -1;
		}
		
		//スキップできるように、count==-1ならループに入らないようにしてある
		while (count!=-1) {

			//ループ回数カウンタ（最初のループが１）
			count++;

			//比をカウンタ倍した値がすでに決まってる投票数以上なら、
			//投票数を更新し、ループを抜ける
			//（比を満たす、今より大きな数を出していけば答えは出るだろ）
			if ((T[i] * count >= voteT) && (A[i] * count >= voteA)) {
				voteT = T[i] * count;
				voteA = A[i] * count;
				break;
			}
		}
	}

	//結果を表示する
	printf("%lld",voteT+voteA);

    return 0;
}