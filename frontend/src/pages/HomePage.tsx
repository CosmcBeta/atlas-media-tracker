function HomePage() {
	return (
		<div>
			<div>Welcome Content</div>

			<ContinueCarousel />

			<RecentlyAdded />

			<FavoritesList />
		</div>
	);
}

function ContinueCarousel() {
	return <div>Continue Carousel</div>;
}

function RecentlyAdded() {
	return <div>Recently Added</div>;
}

function FavoritesList() {
	return <div>Favorites List</div>;
}

export default HomePage;
