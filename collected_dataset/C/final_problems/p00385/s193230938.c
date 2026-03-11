#include <stdio.h>

typedef struct
{
	int flip;//裏表??boolでもOK??
	long long rotate;
} Query;

long long n, k, q;

Query tree[(1 << 20) * 2];//20回左シフトして2倍??

void  build(long long _n);
Query merge(Query a, Query b);
void  update(Query data, long long i);
Query sum(void);

int main()
{
	long long _n;
	long long a;

	//区間の個数、カードの枚数、命令の個数
	scanf("%lld %lld %lld", &k, &_n, &q);

	build(_n);

	for (long long i = 0; i < _n; i++) 
	{
		scanf("%lld", &a);//カード（山札）の数字
		Query data = { 0, 0 };//構造体宣言初期化

		if (a == 0)
		{
			data.flip = 1;//裏返ったらflipを1にする
		}
		else
		{
			data.rotate = -a;
		}

		update(data, i);
	}

	for (long long i = 0; i < q; i++) 
	{
		long long left, right;
		Query l, r;
		scanf("%lld %lld", &left, &right);
		left--;
		right--;

		l = tree[left + n - 1];
		r = tree[right + n - 1];

		update(l, right);
		update(r, left);


		Query total = sum();

		long long num;
		if (total.rotate < 0) total.rotate = k + total.rotate;
		num = 1 + total.rotate;

		if (total.flip) 
		{
			if (num == 1)
			{
				num *= -1;
			}
			else
			{
				num = ((k + 2) - num) * -1;
			}
		}

		printf("%lld\n", num);
	}
	return 0;
}

void  build(long long _n)//カードの枚数引数
{
	n = 1;//グローバル変数

	while (n < _n) //n<3である限り続く
	{
		n *= 2;//n==4になる
	}

	for (long long i = 0; i < 2 * n; i++) 
	{
		tree[i] = (Query){ 0, 0 }; //tree[0]～tree[7]まで作成
	}

	return;
}

Query merge(Query a, Query b)
{
	Query x;

	x.flip = (a.flip + b.flip) % 2;

	if (b.flip)//裏表判別
	{
		x.rotate = (b.rotate - a.rotate) % k;//表
	}
	else 
	{
		x.rotate = (a.rotate + b.rotate) % k;//裏
	}

	return x;
}

void update(Query data, long long i)
{
	i += n - 1;//

	tree[i] = data;//グローバルtree[]にdata格納

	while (i != 0) 
	{
		i = (i - 1) / 2;
		tree[i] = merge(tree[i * 2 + 1], tree[i * 2 + 2]);
	}

	return;
}

Query sum(void)
{
	return tree[0];
}

