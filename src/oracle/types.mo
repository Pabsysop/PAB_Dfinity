import DistanceMetric "mo:base/Float";
import Math "mo:base/Float";
import Nat "mo:base/Nat";
import Nat32 "mo:base/Nat32";
import Prim "mo:prim";
import Principal "mo:base/Principal";

module {

  public let pi : Float = 3.14;
  public let pow : Float = 2.718;
  public type Disability = { #hearing; #sight; #mobility; #other };
  public type Age = Nat; /* bucket by decade */
  //public type Gender = {#male; #female; #other};
  public type RequestType = {#grocery; #pharmacy};
  public type Expiry = { time: Nat};

  public type Location = {
    lat: Float;
    lng: Float
  };
  public type Name = {
    first: Text;
    last: Text;
  };

  public type Status = {
    #active/*: Expiry*/;
    #accepted: HelperId;
    #confirmed;
  };

  public type Request = {
    requestType: RequestType;
    requestLocation: Location;
    note: Text;
    items: [Text];
    reward: Nat; // shields
  };
  public type RequestState = {
    info : Request;
    status : Status;
    user: UserId;
  };

  public type UserAuth = {
    id: Name;
    pKey: Location;
    sig: Age;
    oauth: ?Disability;
  };
  public type UserGene = {
    x: Name;
    y: Location;
    dna: blob;
  };
  public type UserProfile = {
    name: Name;
    location: Location;
    age: Age;
    disability: ?Disability;
    address: [Text];
    email: Text;
  };

  public type UserNFT = {
    title: Name;
    location: Location;
    radiusKm: Float;
    hash: Text;
    address: Text;
  };

  public type UserPriviledge = {
    title: Name;
    location: Location;
    radiusKm: Float;
    hash: Text;
    address: Text;
  };

  public type UserContent = {
    title: Name;
    location: Location;
    radiusKm: Float;
    hash: Text;
    address: Text;
  };

  public type UserConnections = {
    title: Name;
    location: Location;
    radiusKm: Float;
    hash: Text;
    address: Text;
  };

  public type UserFace = {
    title: Name;
    location: Location;
    radiusKm: Float;
    hash: Text;
    address: Text;
  };

  public type BoardProfile = {
    name: Name;
    location: Location;
    age: Age;
    disability: ?Disability;
    address: [Text];
    email: Text;
  };

  public type BoardAuth = {
  }
  public type BoardSponsor = {
  }
  public type BoardContent = {
  }
  public type BoardMember = {
  }
  public type BoardScene = {
  }

  // support for Hashing
  public module CallerId = {
     public func eq(id1 : Principal, id2 : Principal) : Bool { id1 == id2 };
     public func hash(id : Principal) : Nat32 { Principal.hash(id)};
  };
  public type UserId = Principal;
  public type RequestId = Nat;
  public let UserId = CallerId;

  public module RequestId = {
     public func eq(id1 : Nat, id2 : Nat) : Bool { id1 == id2 };
     public func hash(id : Nat) : Nat32 { Nat32.fromNat(id); };
  };

public func deg2rad(deg: Float): Float {return deg * (pi/180)};

public func getDistanceFromLatLng(l1: Location, l2: Location) : Float {
  var r : Float = 6371; // radius of the earth in km
  var dlat1 : Float = deg2rad(l1.lat);
  var dlat2 : Float = deg2rad(l2.lat);
  var lat_dif = dlat2 - dlat1;
  var lng_dif = deg2rad(l2.lng-l1.lng);
  var MeanLatitude : Float = (dlat1+dlat2)/2;
  var d : Float = r*((lat_dif**2+(Prim.cos(MeanLatitude)*lng_dif)**2)**0.5);
  return d //
  }
}
