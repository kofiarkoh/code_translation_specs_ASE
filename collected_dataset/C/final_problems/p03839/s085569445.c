#include <stdio.h>
#define max(X, Y) ((X) > (Y) ? (X) : (Y))
typedef long long ll;

ll a[100005];
ll all_sum[100005];
ll m_sum[100005];

int main()
{
	ll n, k;
	ll ans = 0;

	scanf("%lld %lld", &n, &k);
	for (int i = 1; i <= n; i++) {
		scanf("%lld", &a[i]);
	}

	for (int i = 1; i <= n; i++) {
		all_sum[i] = all_sum[i - 1] + a[i];
		m_sum[i] = m_sum[i - 1] + max(a[i], 0);
	}

	for (ll i = 1; i + k - 1 <= n; i++) {
		//black
		ans = max(ans, (m_sum[i - 1] - m_sum[0]) + (m_sum[n] - m_sum[i + k - 1]) + all_sum[i + k - 1] - all_sum[i - 1]);

		//white
		ans = max(ans, (m_sum[i - 1] - m_sum[0]) + (m_sum[n] - m_sum[i + k - 1]));

	}

	printf("%lld\n", ans);

	return 0;
}
